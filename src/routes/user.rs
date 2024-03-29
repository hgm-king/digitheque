use crate::{
    models::{self, user::ExpandedUser},
    routes,
    utils::now,
    Context, ExpandedUserRejection, NotAuthorized, NotFound, OldCookie, ResourceError, ServerError,
    GLOBAL_PRELUDE,
};
use diesel::result::{DatabaseErrorKind, Error::DatabaseError};
use warp::{
    filters::{self, BoxedFilter},
    reject, Filter,
};

pub fn logout() -> BoxedFilter<()> {
    warp::path("logout")
        .and(warp::path::end())
        .and(warp::get())
        .and(read_cookie())
        .and_then(clear_session)
        .untuple_one()
        .boxed()
}

pub fn signup() -> BoxedFilter<(
    Context,
    models::user::ExpandedUser,
    models::workspace::WorkspaceWithChildren,
)> {
    warp::path("signup")
        .and(warp::path::end())
        .and(warp::post())
        .and(filters::ext::get::<Context>())
        .and(warp::body::form::<models::user::NewUserApi>())
        .and_then(insert_new_user)
        .untuple_one()
        .and_then(with_new_session)
        .untuple_one()
        .and_then(routes::workspace::insert_root_workspace)
        .untuple_one()
        .boxed()
}

pub fn login() -> BoxedFilter<(
    Context,
    models::user::ExpandedUser,
    models::workspace::WorkspaceWithChildren,
)> {
    warp::path("login")
        .and(warp::path::end())
        .and(warp::post())
        .and(filters::ext::get::<Context>())
        .and(warp::body::form::<models::user::UserCredentialsApi>())
        .and_then(with_user_by_credentials)
        .untuple_one()
        .and_then(with_new_session)
        .untuple_one()
        .and_then(routes::workspace::with_root_workspace)
        .untuple_one()
        .boxed()
}

pub fn get_by_cookie() -> BoxedFilter<(
    Context,
    models::user::ExpandedUser,
    models::workspace::WorkspaceWithChildren,
)> {
    warp::path::end()
        .and(warp::get())
        .and(authenticate_cookie())
        .and_then(routes::workspace::with_root_workspace)
        .untuple_one()
        .boxed()
}

async fn with_user_by_credentials(
    context: Context,
    credentials: models::user::UserCredentialsApi,
) -> Result<(Context, models::user::User), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    tracing::info!("Looking for user {}", credentials.username);
    let user = models::user::User::read_by_credentials(&mut conn, credentials)
        .map_err(|_| reject::custom(NotFound))?;
    Ok((context, user))
}

async fn insert_new_user(
    context: Context,
    new_user: models::user::NewUserApi,
) -> Result<(Context, models::user::User), warp::Rejection> {
    tracing::debug!("Saving User");
    let mut conn = context.db_conn.get_conn();

    let user = models::user::NewUser::new(new_user.into())
        .insert(&mut conn)
        .map_err(|e| {
            tracing::error!("{:?}", e);
            match e {
                DatabaseError(DatabaseErrorKind::UniqueViolation, _) => {
                    reject::custom(ResourceError {
                        message: String::from("This user already exists."),
                    })
                }
                err => {
                    tracing::error!("{:?}", err);
                    reject::custom(ServerError {
                        message: err.to_string(),
                    })
                }
            }
        })?;
    tracing::debug!("Saved User");
    Ok((context, user))
}

pub fn signup_form() -> BoxedFilter<()> {
    warp::path("signup")
        .and(warp::path::end())
        .and(warp::get())
        .boxed()
}

pub fn login_form() -> BoxedFilter<()> {
    warp::path("login")
        .and(warp::path::end())
        .and(warp::get())
        .boxed()
}

async fn with_new_session(
    context: Context,
    user: models::user::User,
) -> Result<(Context, models::user::ExpandedUser), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    models::session::delete_by_user_id(&mut conn, user.id).map_err(|err| {
        tracing::error!("{:?}", err);
        warp::reject()
    })?;

    let session = models::session::NewSession::new(user.id)
        .insert(&mut conn)
        .map_err(|_| {
            warp::reject::custom(ResourceError {
                message: String::from("You cannot log in on more than one device."),
            })
        })?;

    let expanded_user = ExpandedUser { user, session };
    Ok((context, expanded_user))
}

async fn clear_session(
    context: Context,
    session: models::session::Session,
) -> Result<(), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    models::session::delete(&mut conn, &session).map_err(|_| warp::reject::custom(NotFound))?;
    Ok(())
}

async fn with_user_from_cookie(
    context: Context,
    session_id: i32,
) -> Result<(Context, models::user::ExpandedUser), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    tracing::info!("Session ID: {}", session_id);
    let expanded_user = models::user::read_user_by_session(&mut conn, session_id)
        .map_err(|_| warp::reject::custom(NotAuthorized))?;
    tracing::info!(
        "Recognized user {:?} from {:?}",
        expanded_user.user,
        expanded_user.session
    );

    if expanded_user.session.valid_until < now() {
        models::session::delete(&mut conn, &expanded_user.session)
            .map_err(|_| warp::reject::custom(NotFound))?;
        return Err(warp::reject::custom(OldCookie));
    }

    Ok((context, expanded_user))
}

async fn reject_with_user(
    context: Context,
    session_id: i32,
) -> Result<(Context, models::user::ExpandedUser), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();
    tracing::error!("Adding user object into this rejection");
    let expanded_user =
        models::user::read_user_by_session(&mut conn, session_id).map_err(|_| {
            warp::reject::custom(ExpandedUserRejection {
                expanded_user: None,
            })
        })?;

    tracing::error!("We have a logged in user");

    Err(warp::reject::custom(ExpandedUserRejection {
        expanded_user: Some(expanded_user),
    }))
}

async fn reject_without_user(_context: Context) -> Result<(), warp::Rejection> {
    tracing::error!("Rejecting without user!");
    Err(warp::reject::custom(ExpandedUserRejection {
        expanded_user: None,
    }))
}

async fn with_session_from_cookie(
    context: Context,
    session_id: i32,
) -> Result<(Context, models::session::Session), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    let session = models::session::read_by_id(&mut conn, session_id)
        .map_err(|_| warp::reject::custom(NotFound))?;

    Ok((context, session))
}

pub fn authenticate_cookie() -> BoxedFilter<(Context, models::user::ExpandedUser)> {
    warp::any()
        .and(filters::ext::get::<Context>())
        .and(warp::cookie("session"))
        .and_then(with_user_from_cookie)
        .untuple_one()
        .boxed()
}

pub fn logged_in_rejection() -> BoxedFilter<(
    Context,
    models::user::ExpandedUser,
    models::workspace::WorkspaceWithChildren,
)> {
    warp::any()
        .and(filters::ext::get::<Context>())
        .and(warp::cookie("session"))
        .and_then(reject_with_user)
        .untuple_one()
        .and_then(routes::workspace::with_root_workspace)
        .untuple_one()
        .boxed()
}

pub fn logged_out_rejection() -> BoxedFilter<()> {
    warp::any()
        .and(filters::ext::get::<Context>())
        .and_then(reject_without_user)
        .untuple_one()
        .boxed()
}

pub fn read_cookie() -> BoxedFilter<(Context, models::session::Session)> {
    warp::any()
        .and(filters::ext::get::<Context>())
        .and(warp::cookie("session"))
        .and_then(with_session_from_cookie)
        .untuple_one()
        .boxed()
}

pub fn update_style() -> BoxedFilter<(Context, models::user::ExpandedUser, Option<String>)> {
    warp::path("stylesheet")
        .and(warp::path::end())
        .and(
            warp::post()
                .and(routes::user::authenticate_cookie())
                .and(warp::body::form::<models::user::UpdateStyleApi>())
                .and_then(update_user_style)
                .untuple_one()
                .or(warp::get()
                    .and(routes::user::authenticate_cookie())
                    .map(|context, user| (context, user, None))
                    .untuple_one()),
        )
        .unify()
        .boxed()
}

pub fn update_prelude() -> BoxedFilter<(Context, models::user::ExpandedUser, Option<String>)> {
    warp::path("prelude")
        .and(warp::path::end())
        .and(
            warp::post()
                .and(routes::user::authenticate_cookie())
                .and(warp::body::form::<models::user::UpdatePreludeApi>())
                .and_then(update_user_prelude)
                .untuple_one()
                .or(warp::get()
                    .and(routes::user::authenticate_cookie())
                    .map(|context, user| (context, user, None))
                    .untuple_one()),
        )
        .unify()
        .boxed()
}

async fn update_user_prelude(
    context: Context,
    mut expanded_user: models::user::ExpandedUser,
    new_prelude: models::user::UpdatePreludeApi,
) -> Result<(Context, models::user::ExpandedUser, Option<String>), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    let input = &format!(
        r#"
            {}
            {}
            "#,
        GLOBAL_PRELUDE,
        new_prelude.prelude.clone()
    );
    let mut env = bebop_lang::lisp::env::init_env();
    let v = bebop_lang::lisp::lisp(&mut env, input);

    expanded_user.user.prelude = Some(new_prelude.prelude);
    expanded_user.user.update(&mut conn).map_err(|e| {
        tracing::error!("{:?}", e);
        reject::custom(ServerError {
            message: e.to_string(),
        })
    })?;

    Ok((context, expanded_user, Some(v)))
}

async fn update_user_style(
    context: Context,
    mut expanded_user: models::user::ExpandedUser,
    new_style: models::user::UpdateStyleApi,
) -> Result<(Context, models::user::ExpandedUser, Option<String>), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    expanded_user.user.style = Some(new_style.style);
    expanded_user.user.update(&mut conn).map_err(|e| {
        tracing::error!("{:?}", e);
        reject::custom(ServerError {
            message: e.to_string(),
        })
    })?;

    Ok((context, expanded_user, Some(String::from("Style updated!"))))
}

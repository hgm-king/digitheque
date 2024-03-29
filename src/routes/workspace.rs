use crate::{models, routes, Context, ServerError};
use warp::{filters::BoxedFilter, reject, Filter};

pub fn workspace() -> BoxedFilter<(
    Context,
    models::user::ExpandedUser,
    models::workspace::WorkspaceWithChildren,
)> {
    warp::path("workspace")
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(warp::get())
        .and(routes::user::authenticate_cookie())
        .and_then(with_workspace)
        .untuple_one()
        .boxed()
}

pub fn edit_page() -> BoxedFilter<(
    Context,
    models::user::ExpandedUser,
    models::workspace::WorkspaceWithChildren,
)> {
    warp::path("workspace")
        .and(warp::path::param::<i32>())
        .and(warp::path("edit"))
        .and(warp::path::end())
        .and(warp::get())
        .and(routes::user::authenticate_cookie())
        .and_then(with_workspace)
        .untuple_one()
        .boxed()
}

pub fn new() -> BoxedFilter<(
    Context,
    models::user::ExpandedUser,
    models::workspace::WorkspaceWithChildren,
)> {
    warp::path("workspace")
        .and(warp::path::param::<i32>())
        .and(warp::path("new"))
        .and(warp::path::end())
        .and(warp::post())
        .and(routes::user::authenticate_cookie())
        .and(warp::body::form::<models::workspace::NewWorkspaceApi>())
        .and_then(with_new_workspace)
        .untuple_one()
        .and_then(with_workspace)
        .untuple_one()
        .boxed()
}

pub fn edit() -> BoxedFilter<(
    Context,
    models::user::ExpandedUser,
    models::workspace::WorkspaceWithChildren,
)> {
    warp::path("workspace")
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(warp::post())
        .and(routes::user::authenticate_cookie())
        .and(warp::body::form::<models::workspace::EditWorkspaceApi>())
        .and_then(with_update_workspace)
        .untuple_one()
        .and_then(with_workspace)
        .untuple_one()
        .boxed()
}

pub fn publish() -> BoxedFilter<(
    Context,
    models::user::ExpandedUser,
    models::workspace::WorkspaceWithChildren,
)> {
    warp::path("workspace")
        .and(warp::path::param::<i32>())
        .and(warp::path("publish"))
        .and(warp::path::end())
        .and(warp::post())
        .and(routes::user::authenticate_cookie())
        .and(warp::body::form::<models::workspace::PublishWorkspaceApi>())
        .and_then(with_publish_workspace)
        .untuple_one()
        .and_then(with_workspace)
        .untuple_one()
        .boxed()
}

pub async fn with_root_workspace(
    context: Context,
    expanded_user: models::user::ExpandedUser,
) -> Result<
    (
        Context,
        models::user::ExpandedUser,
        models::workspace::WorkspaceWithChildren,
    ),
    warp::Rejection,
> {
    let mut conn = context.db_conn.get_conn();

    let workspace = models::workspace::WorkspaceWithChildren::read_root_by_user(&mut conn, expanded_user.user.id)
        .map_err(|e| {
            tracing::error!("{:?}", e);
            reject::custom(ServerError {
                message: e.to_string(),
            })
        })?;

    Ok((context, expanded_user, workspace.unwrap()))
}

async fn with_update_workspace(
    id: i32,
    context: Context,
    expanded_user: models::user::ExpandedUser,
    edit_workspace: models::workspace::EditWorkspaceApi,
) -> Result<(i32, Context, models::user::ExpandedUser), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    edit_workspace.update(&mut conn, id).map_err(|e| {
        tracing::error!("{:?}", e);
        reject::custom(ServerError {
            message: e.to_string(),
        })
    })?;

    Ok((id, context, expanded_user))
}

async fn with_publish_workspace(
    id: i32,
    context: Context,
    expanded_user: models::user::ExpandedUser,
    publish_workspace: models::workspace::PublishWorkspaceApi,
) -> Result<(i32, Context, models::user::ExpandedUser), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    publish_workspace.publish(&mut conn, id).map_err(|e| {
        tracing::error!("{:?}", e);
        reject::custom(ServerError {
            message: e.to_string(),
        })
    })?;

    Ok((id, context, expanded_user))
}

pub async fn with_workspace(
    id: i32,
    context: Context,
    expanded_user: models::user::ExpandedUser,
) -> Result<
    (
        Context,
        models::user::ExpandedUser,
        models::workspace::WorkspaceWithChildren,
    ),
    warp::Rejection,
> {
    let mut conn = context.db_conn.get_conn();

    let workspace = models::workspace::WorkspaceWithChildren::read_by_user_and_id(&mut conn, expanded_user.user.id, id)
        .map_err(|e| {
            tracing::error!("{:?}", e);
            reject::custom(ServerError {
                message: e.to_string(),
            })
        })?;

    Ok((context, expanded_user, workspace.unwrap()))
}

pub async fn with_new_workspace(
    parent_id: i32,
    context: Context,
    expanded_user: models::user::ExpandedUser,
    new_workspace: models::workspace::NewWorkspaceApi,
) -> Result<(i32, Context, models::user::ExpandedUser), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    let _new_workspace =
        models::workspace::NewWorkspace::new(new_workspace, expanded_user.user.id, parent_id)
            .insert(&mut conn)
            .map_err(|e| {
                tracing::error!("{:?}", e);
                reject::custom(ServerError {
                    message: e.to_string(),
                })
            })?;

    Ok((parent_id, context, expanded_user))
}

pub async fn insert_root_workspace(
    context: Context,
    expanded_user: models::user::ExpandedUser,
) -> Result<
    (
        Context,
        models::user::ExpandedUser,
        models::workspace::WorkspaceWithChildren,
    ),
    warp::Rejection,
> {
    let mut conn = context.db_conn.get_conn();

    let new_workspace = models::workspace::NewWorkspace::new(
        models::workspace::NewWorkspaceApi {
            name: expanded_user.user.username.clone(),
            description: String::from("Your root workspace is the source of your creation."),
            type_id: models::workspace::WorkspaceType::Root as i32,
            // styles: None,
            // todo_state: None,
            // link_url: None,
            // img_url: None,
            // content: None,
        },
        expanded_user.user.id,
        -1,
    );

    let workspace = new_workspace.insert(&mut conn).map_err(|e| {
        tracing::error!("{:?}", e);
        reject::custom(ServerError {
            message: e.to_string(),
        })
    })?;
    tracing::debug!("Saved Workspace");

    Ok((
        context,
        expanded_user,
        models::workspace::WorkspaceWithChildren {
            workspace,
            children: vec![],
        },
    ))
}
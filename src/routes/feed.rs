use crate::{models::{self, user::ExpandedUser}, routes, Context, ServerError};
use warp::{filters::{self, BoxedFilter}, reject, Filter};

pub fn feed() -> BoxedFilter<(
    Context,
    models::feed::Feed,
)> {
    warp::path::param::<String>()
        .and(warp::path("rss"))
        .and(warp::path::end())
        .and(warp::get())
        .and(filters::ext::get::<Context>())
        .and_then(with_feed)
        .untuple_one()
        .boxed()
}

fn workspace_prefix() -> BoxedFilter<(String,i32,)> {
    warp::path::param::<String>()
        .and(warp::path("workspace"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(warp::get())
        .boxed()
}

pub fn workspace() -> BoxedFilter<(
    Context,
    Option<models::user::ExpandedUser>,
    models::feed::FeedWorkspace,
)> {
    workspace_prefix()
        .and(routes::user::authenticate_cookie())
        .map(|s, id, context, expanded_user| (s, id, context, Some(expanded_user)))
        .or(workspace_prefix().and(filters::ext::get::<Context>()).map(|s, id, c| (s, id, c, None)))
        .unify()
        .untuple_one()
        .and_then(with_workspace)
        .untuple_one()
        .boxed()
}

async fn with_workspace(
    username: String,
    workspace_id: i32,
    context: Context,
    expanded_user: Option<ExpandedUser>
) -> Result<(
    Context,
    Option<ExpandedUser>,
    models::feed::FeedWorkspace,
), warp::Rejection> {
    let mut conn = context.db_conn.get_conn();

    let workspace = models::feed::FeedWorkspace::get_for_user(&mut conn, username, workspace_id).map_err(|e| {
        tracing::error!("{:?}", e);
        reject::custom(ServerError {
            message: e.to_string(),
        })
    })?;

    if workspace.is_none() {
        return Err(warp::reject());
    };

    let workspace = workspace.unwrap();

    tracing::info!("{:?}", workspace.user);

    Ok((context, expanded_user, workspace))
}

async fn with_feed(
    username: String,
    context: Context,
) -> Result<
    (
        Context,
        models::feed::Feed,
    ),
    warp::Rejection,
> {
    let mut conn = context.db_conn.get_conn();

    let feed = models::feed::Feed::get_for_user(&mut conn, username).map_err(|e| {
        tracing::error!("{:?}", e);
        reject::custom(ServerError {
            message: e.to_string(),
        })
    })?;

    if feed.is_none() {
        return Err(warp::reject());
    }

    Ok((context, feed.unwrap()))
}

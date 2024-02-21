use crate::{models, Context, ServerError};
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

pub fn feed_authenticated() -> BoxedFilter<(
    Context,
    Option<models::user::ExpandedUser>,
    models::feed::Feed,
)> {
    feed_prefix()
        .and(routes::user::authenticate_cookie())
        .map(|s, context, expanded_user| (s, context, Some(expanded_user)))
        .or(feed_prefix().and(filters::ext::get::<Context>()).map(|s, c| (s, c, None)))
        .unify()
        .untuple_one()
        .and_then(with_feed)
        .untuple_one()
        .boxed()
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

    Ok((context, feed))
}

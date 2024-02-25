use crate::{models, views, Context};

pub async fn feed(
    _context: Context,
    feed: models::feed::Feed,
) -> Result<impl warp::Reply, warp::Rejection> {
    let rss_feed = feed.to_rss_channel();
    
    Ok(warp::reply::with_header(
        warp::reply::html(rss_feed.to_string()),
        "Content-Type",
        "text/xml"
    ))
}

pub async fn workspace(
    _context: Context,
    expanded_user: Option<models::user::ExpandedUser>,
    workspace: models::feed::FeedWorkspace,
) -> Result<impl warp::Reply, warp::Rejection> {
    let workspace_html = views::feed::workspace_page(expanded_user, workspace);

    Ok(warp::reply::html(workspace_html))
}

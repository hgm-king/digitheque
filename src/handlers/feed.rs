use crate::{models, Context};

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

pub mod user;
pub mod workspace;

use crate::{models, views};

pub async fn index(
    expanded_user: Option<models::user::ExpandedUser>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let landing_html = views::common::landing_page(expanded_user);

    Ok(warp::reply::html(landing_html))
}

pub async fn bebop(
    expanded_user: Option<models::user::ExpandedUser>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let landing_html = views::common::bebop_page(expanded_user);

    Ok(warp::reply::html(landing_html))
}

use crate::{models, views, Context, NotFound, ResourceError};
use std::convert::Infallible;
use warp::{hyper::StatusCode, Rejection, Reply};

pub async fn profile(
    _context: Context,
    expanded_user: models::user::ExpandedUser,
    workspace: models::workspace::WorkspaceWithChildren,
) -> Result<impl warp::Reply, warp::Rejection> {
    let profile_html = views::user::profile_page(expanded_user, workspace);

    Ok(warp::reply::html(profile_html))
}

pub async fn profile_with_cookie(
    _context: Context,
    expanded_user: models::user::ExpandedUser,
    workspace: models::workspace::WorkspaceWithChildren,
) -> Result<impl warp::Reply, warp::Rejection> {
    let cookie_value = format!("session={}; path=/", expanded_user.session.id);
    let profile_html = views::user::profile_page(expanded_user, workspace);

    Ok(warp::reply::with_header(
        warp::reply::html(profile_html),
        "Set-Cookie",
        cookie_value,
    ))
}

// pub fn get_workspaces(
//     context: Context,
//     expanded_user: &models::user::ExpandedUser,
// ) -> Result<Vec<models::workspace::Workspace>, warp::Rejection> {
//     let mut conn = context.db_conn.get_conn();

//     models::page::read_pages_by_user_id(&mut conn, expanded_user.user.id).map_err(|e| {
//         log::error!("{:?}", e);
//         warp::reject::not_found()
//     })
// }

pub async fn logout() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::with_header(
        warp::reply::html(views::common::landing_page(None)),
        "Set-Cookie",
        format!("session=; Path=/"),
    ))
}

pub async fn signup_form() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html(views::auth::signup_form(None)))
}

pub async fn login_form() -> Result<impl warp::Reply, Infallible> {
    Ok(warp::reply::html(views::auth::login_form(None)))
}

pub async fn login_error(err: Rejection) -> Result<impl Reply, Rejection> {
    tracing::error!("{:?}", err);
    if let Some(NotFound) = err.find::<NotFound>() {
        let html = views::auth::login_form(Some(String::from("Error: Invalid login credentials")));
        let html = warp::reply::html(html);
        Ok(warp::reply::with_status(html, StatusCode::NOT_FOUND))
    } else {
        Err(err)
    }
}

pub async fn signup_error(err: Rejection) -> Result<impl Reply, Rejection> {
    tracing::error!("{:?}", err);
    if let Some(e) = err.find::<ResourceError>() {
        let html = views::auth::login_form(Some(e.message.clone()));
        let html = warp::reply::html(html);
        Ok(warp::reply::with_status(html, StatusCode::NOT_FOUND))
    } else {
        Err(err)
    }
}

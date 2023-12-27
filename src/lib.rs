pub mod api;
pub mod config;
pub mod db_conn;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod schema;
pub mod utils;
pub mod views;

#[macro_use]
extern crate diesel;

use std::sync::Arc;
use models::user::ExpandedUser;
use warp::{hyper::StatusCode, reject, Rejection, Reply};

#[derive(Debug)]
struct NotFound;
impl reject::Reject for NotFound {}

#[derive(Debug)]
struct NotAuthorized;
impl reject::Reject for NotAuthorized {}

#[derive(Debug)]
struct OldCookie;
impl reject::Reject for OldCookie {}

#[derive(Debug)]
pub struct ResourceError {
    message: String,
}
impl reject::Reject for ResourceError {}

#[derive(Debug)]
pub struct ServerError {
    message: String,
}
impl reject::Reject for ServerError {}

#[derive(Clone, Debug)]
pub struct ExpandedUserRejection {
    expanded_user: Option<ExpandedUser>,
}
impl reject::Reject for ExpandedUserRejection {}

#[derive(Clone, Debug)]
pub struct Context {
    pub config: Arc<config::Config>,
    pub db_conn: Arc<db_conn::DbConn>,
}


impl Context {
    pub fn new(config: Arc<config::Config>, db_conn: Arc<db_conn::DbConn>) -> Self {
        Context {
            config: config,
            db_conn,
        }
    }
}

pub async fn handle_rejections(err: Rejection) -> Result<impl Reply, Rejection> {
    let expanded_user = err.find::<ExpandedUserRejection>().map(|eu| eu.clone().expanded_user).unwrap_or(None);

    tracing::error!("{:?}", err);
    tracing::error!("Handling rejection for user {:?}", expanded_user);

    if expanded_user.is_some() {
        if let Some(e) = err.find::<ResourceError>() {
            let code = StatusCode::BAD_REQUEST;
            let html = views::error::error_page(code, &e.message, expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(_) = err.find::<NotAuthorized>() {
            let code = StatusCode::FORBIDDEN;
            let html = views::error::error_page(code, "You are not authorized to do this", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        }
        // else if let Some(_) = err.find::<OldCookie>() {
        //     Ok(Box::new(warp::reply::with_header(
        //         warp::reply::html(views::body::index("Your session has expired")),
        //         "Set-Cookie",
        //         format!("session=; Path=/"),
        //     )))
        // }
        else if let Some(_) = err.find::<reject::MissingCookie>() {
            let code = StatusCode::FORBIDDEN;
            let html = views::error::error_page(code, "You are not logged in", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(NotFound) = err.find::<NotFound>() {
            let code = StatusCode::NOT_FOUND;
            let html = views::error::error_page(code, "We could not locate this resource", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
            let message = e.to_string();
            let code = StatusCode::NOT_FOUND;
            let html = views::error::error_page(code, &message, expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(_) = err.find::<reject::UnsupportedMediaType>() {
            let code = StatusCode::UNSUPPORTED_MEDIA_TYPE;
            let html = views::error::error_page(code, "UNSUPPORTED MEDIA TYPE", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
            let code = StatusCode::METHOD_NOT_ALLOWED;
            let html = views::error::error_page(code, "METHOD NOT ALLOWED", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else {
            let code = StatusCode::NOT_FOUND;
            let html = views::error::error_page(code, "RESOURCE NOT FOUND", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        }
    } else {
        if let Some(e) = err.find::<ResourceError>() {
            let code = StatusCode::BAD_REQUEST;
            let html = views::error::error_page(code, &e.message, expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(_) = err.find::<NotAuthorized>() {
            let code = StatusCode::FORBIDDEN;
            let html = views::error::error_page(code, "You are not authorized to do this", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        }
        // else if let Some(_) = err.find::<OldCookie>() {
        //     Ok(Box::new(warp::reply::with_header(
        //         warp::reply::html(views::body::index("Your session has expired")),
        //         "Set-Cookie",
        //         format!("session=; Path=/"),
        //     )))
        // }
        // else if let Some(_) = err.find::<reject::MissingCookie>() {
        //     let code = StatusCode::FORBIDDEN;
        //     let html = views::error::error_page(code, "You are not logged in", expanded_user);
        //     Ok(warp::reply::with_status(warp::reply::html(html), code))
        // } 
        else if let Some(NotFound) = err.find::<NotFound>() {
            let code = StatusCode::NOT_FOUND;
            let html = views::error::error_page(code, "We could not locate this resource", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
            let message = e.to_string();
            let code = StatusCode::NOT_FOUND;
            let html = views::error::error_page(code, &message, expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(_) = err.find::<reject::UnsupportedMediaType>() {
            let code = StatusCode::UNSUPPORTED_MEDIA_TYPE;
            let html = views::error::error_page(code, "UNSUPPORTED MEDIA TYPE", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
            let code = StatusCode::METHOD_NOT_ALLOWED;
            let html = views::error::error_page(code, "METHOD NOT ALLOWED", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        } else {
            let code = StatusCode::NOT_FOUND;
            let html = views::error::error_page(code, "RESOURCE NOT FOUND", expanded_user);
            Ok(warp::reply::with_status(warp::reply::html(html), code))
        }
    }

    //  else if let Some(_) = err.find::<reject::MethodNotAllowed>() {
    //     tracing::info!("Passing MethodNotAllowed error through!");
    //     Err(err)
    // } else if err.is_not_found() {
    //     tracing::info!("Passing 404 error through!");
    //     Err(err)
    // } else {
    //     // We should have expected this... Just log and say its a 500
    //     tracing::error!("unhandled rejection: {:?}", err);
    //     let code = StatusCode::INTERNAL_SERVER_ERROR;
    //     let html = views::error::error_page(code, "UNHANDLED_ERROR");
    //     Ok(warp::reply::with_status(warp::reply::html(html), code))
    // }
}
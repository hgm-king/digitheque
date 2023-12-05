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

use std::{convert::Infallible, sync::Arc};
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
    tracing::error!("{:?}", err);
    if let Some(e) = err.find::<ResourceError>() {
        let code = StatusCode::BAD_REQUEST;
        let html = views::error::error_page(code, &e.message);
        Ok(warp::reply::with_status(warp::reply::html(html), code))
    } else if let Some(_) = err.find::<NotAuthorized>() {
        let code = StatusCode::FORBIDDEN;
        let html = views::error::error_page(code, "You are not authorized to do this");
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
        let html = views::error::error_page(code, "You are not logged in");
        Ok(warp::reply::with_status(warp::reply::html(html), code))
    } else if let Some(NotFound) = err.find::<NotFound>() {
        let code = StatusCode::NOT_FOUND;
        let html = views::error::error_page(code, "We could not locate this resource");
        Ok(warp::reply::with_status(warp::reply::html(html), code))
    } else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
        let message = e.to_string();
        let code = StatusCode::NOT_FOUND;
        let html = views::error::error_page(code, &message);
        Ok(warp::reply::with_status(warp::reply::html(html), code))
    } else if let Some(_) = err.find::<reject::UnsupportedMediaType>() {
        let code = StatusCode::UNSUPPORTED_MEDIA_TYPE;
        let html = views::error::error_page(code, "UNSUPPORTED_MEDIA_TYPE");
        Ok(warp::reply::with_status(warp::reply::html(html), code))
    } else {
        tracing::info!("Passing error through!");
        Err(err)
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

pub async fn terminal_error_handler(err: Rejection) -> Result<impl Reply, Infallible> {
    let code;
    let message;

    if let Some(_) = err.find::<reject::MethodNotAllowed>() {
        code = StatusCode::METHOD_NOT_ALLOWED;
        message = "METHOD_NOT_ALLOWED";
    } else if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        message = "RESOURCE NOT FOUND";
    } else {
        // We should have expected this... Just log and say its a 500
        tracing::error!("unhandled rejection: {:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
        message = "UNHANDLED_REJECTION";
    }

    let html = warp::reply::html(views::error::error_page(code, message));
    Ok(warp::reply::with_status(html, code))
}

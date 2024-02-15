pub mod assets;
pub mod user;
pub mod workspace;

use crate::models;
use warp::{filters::BoxedFilter, Filter};

pub fn index() -> BoxedFilter<(Option<models::user::ExpandedUser>,)> {
    warp::path::end()
        .and(warp::get())
        .and(user::authenticate_cookie())
        .map(|_, expanded_user| (Some(expanded_user)))
        .or(warp::path::end().and(warp::get()).map(|| (None)))
        .unify()
        .boxed()
}

pub fn bebop() -> BoxedFilter<(Option<models::user::ExpandedUser>,)> {
    warp::path("bebop.html")
        .and(warp::path::end())
        .and(warp::get())
        .and(user::authenticate_cookie())
        .map(|_, expanded_user| (Some(expanded_user)))
        .or(warp::any().map(|| (None)))
        .unify()
        .boxed()
}

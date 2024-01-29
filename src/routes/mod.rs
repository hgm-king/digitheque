pub mod assets;
pub mod user;
pub mod workspace;

use crate::{models, Context};
use warp::{
    filters::{self, BoxedFilter},
    reject, Filter,
};

pub fn index_logged_out() -> BoxedFilter<(Option<models::user::ExpandedUser>,)> {
    warp::path::end()
        .and(warp::get())
        .map(|| (None) )
        .boxed()
}

pub fn index_logged_in() -> BoxedFilter<(Option<models::user::ExpandedUser>,)> {
    warp::path::end()
        .and(warp::get())
        .and(user::authenticate_cookie())
        .map( |_, expanded_user| (Some(expanded_user)))
        .boxed()
}

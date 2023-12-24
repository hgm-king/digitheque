use warp::{filters::BoxedFilter, fs::File, Filter};

pub fn get_static() -> BoxedFilter<(File,)> {
    warp::fs::dir("./static").boxed()
}

pub fn about() -> BoxedFilter<()> {
    warp::path("about").and(warp::path::end()).boxed()
}
#[macro_export]
macro_rules! assets_api {
    () => {
        routes::assets::get_static()
            //.recover(handle_rejection)
            .with(warp::trace::named("assets"))
    };
}

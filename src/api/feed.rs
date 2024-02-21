#[macro_export]
macro_rules! feed_api {
    () => {
        routes::feed::feed().and_then(handlers::feed::feed)
            .with(warp::trace::named("feed"))
    };
}

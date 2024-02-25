#[macro_export]
macro_rules! feed_api {
    () => {
        routes::feed::feed().and_then(handlers::feed::feed)
        .or(routes::feed::workspace().and_then(handlers::feed::workspace))
            .with(warp::trace::named("feed"))
    };
}

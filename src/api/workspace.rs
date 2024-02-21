#[macro_export]
macro_rules! workspace_api {
    () => {
        routes::workspace::workspace()
            .and_then(handlers::workspace::workspace)
            .or(routes::workspace::new().and_then(handlers::workspace::workspace))
            .or(routes::workspace::edit().and_then(handlers::workspace::workspace))
            .or(routes::workspace::publish().and_then(handlers::workspace::workspace))
            .or(routes::workspace::edit_page().and_then(handlers::workspace::edit_workspace))
            .with(warp::trace::named("workspace"))
    };
}

#[macro_export]
macro_rules! workspace_api {
    () => {
        routes::workspace::workspace()
            .and_then(handlers::workspace::workspace)
            .or(routes::workspace::create_workspace().and_then(handlers::workspace::workspace))
            .or(routes::workspace::edit_workspace().and_then(handlers::workspace::workspace))
            .or(routes::workspace::publish_workspace().and_then(handlers::workspace::workspace))
            .or(routes::workspace::workspace_edit().and_then(handlers::workspace::edit_workspace))
            .with(warp::trace::named("workspace"))
    };
}

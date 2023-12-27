#[macro_export]
macro_rules! workspace_api {
    () => {
        
        routes::workspace::workspace()
            .and_then(handlers::workspace::workspace)
            .or(
                routes::workspace::create_workspace()
                    .and_then(handlers::workspace::workspace)
            )
            .with(warp::trace::named("workspace"))
    };
}

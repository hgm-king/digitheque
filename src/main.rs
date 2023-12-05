use std::{
    convert::Infallible,
    net::{SocketAddr, TcpListener},
    sync::Arc,
};
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use warp::{
    hyper::{service::make_service_fn, Server},
    reply, Filter,
};

use digitheque::{
    assets_api, config::Config, db_conn::DbConn, handle_rejections, handlers, routes,
    terminal_error_handler, user_api, Context,
};

#[tokio::main]
async fn main() -> Result<(), ()> {
    tracing_subscriber::fmt::init();

    let config = Arc::new(Config::new(false));
    let db_conn = Arc::new(DbConn::new(&config.db_path));
    let context = Context::new(config.clone(), db_conn.clone());

    let end = assets_api!()
        .or(user_api!())
        .map(|reply| reply::with_header(reply, "Access-Control-Allow-Origin", "*"))
        .recover(terminal_error_handler);

    let app = make_service_fn(move |_| {
        let context = context.clone();
        let end = end.clone();

        async move {
            Ok::<_, Infallible>(
                ServiceBuilder::new()
                    .layer(AddExtensionLayer::new(context))
                    .service(warp::service(end)),
            )
        }
    });

    let socket_address = config
        .clone()
        .app_addr
        .parse::<SocketAddr>()
        .expect("Addr to parse correctly");

    tracing::info!("👂 Listening on {}", socket_address);

    let listener = TcpListener::bind(socket_address).unwrap();
    Server::from_tcp(listener)
        .unwrap()
        .serve(app)
        .await
        .expect("Server to start normally");

    Ok(())
}

use std::{
    convert::Infallible,
    net::{SocketAddr, TcpListener},
    sync::Arc,
};
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tracing_subscriber::fmt::format::FmtSpan;
use warp::{
    hyper::{server::conn::AddrIncoming, service::make_service_fn, Server},
    reply, Filter,
};

use digitheque::{
    assets_api,
    config::Config,
    db_conn::DbConn,
    handle_rejections, handlers, routes, user_api,
    utils::{load_certs, load_private_key},
    workspace_api, Context,
};
use hyper_rustls::TlsAcceptor;

#[tokio::main]
async fn main() -> Result<(), ()> {
    let filter = std::env::var("RUST_LOG")
        .unwrap_or_else(|_| "digitheque=info,tracing=info,warp=trace".to_owned());

    // Configure the default `tracing` subscriber.
    // The `fmt` subscriber from the `tracing-subscriber` crate logs `tracing`
    // events to stdout. Other subscribers are available for integrating with
    // distributed tracing systems such as OpenTelemetry.
    tracing_subscriber::fmt()
        // Use the filter we built above to determine which traces to record.
        .with_env_filter(filter)
        // Record an event when each span closes. This can be used to time our
        // routes' durations!
        .with_span_events(FmtSpan::CLOSE)
        .init();

    let config = Arc::new(Config::new(false));
    let db_conn = Arc::new(DbConn::new(&config.db_path));
    let context = Context::new(config.clone(), db_conn.clone());

    let end = assets_api!()
        .or(user_api!())
        .or(workspace_api!())
        .or((routes::index_logged_in().and_then(handlers::index))
            .or(routes::index_logged_out().and_then(handlers::index)))
        .or(
            // surface logged in data to errors
            routes::user::logged_in_rejection().and_then(handlers::user::profile),
        )
        .map(|reply| reply::with_header(reply, "Access-Control-Allow-Origin", "*"))
        .recover(handle_rejections)
        .with(warp::trace::request());

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

    tracing::info!("🔐 TLS Enabled!");
    // Load public certificate.
    let certs = load_certs(&config.cert_path.clone().unwrap()).unwrap();
    // Load private key.
    let key = load_private_key(&config.key_path.clone().unwrap()).unwrap();
    // Build TLS configuration.
    // Create a TCP listener via tokio.
    let incoming = AddrIncoming::bind(&socket_address).unwrap();
    let acceptor = TlsAcceptor::builder()
        .with_single_cert(certs, key)
        .unwrap()
        .with_all_versions_alpn()
        .with_incoming(incoming);
    Server::builder(acceptor).serve(app).await.unwrap();

    // let listener = TcpListener::bind(socket_address).unwrap();
    // Server::from_tcp(listener)
    //     .unwrap()
    //     .serve(app)
    //     .await
    //     .expect("Server to start normally");

    Ok(())
}

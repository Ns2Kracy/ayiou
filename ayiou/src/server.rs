use axum::{Router, http::HeaderName};
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    propagate_header::PropagateHeaderLayer,
    request_id::{MakeRequestUuid, SetRequestIdLayer},
};

use crate::{api, shutdown::shutdown_signal};

pub async fn run_server() -> anyhow::Result<()> {
    let app = Router::new()
        .merge(api::router())
        .layer(CompressionLayer::new())
        .layer(PropagateHeaderLayer::new(HeaderName::from_static(
            "x-request-id",
        )))
        .layer(SetRequestIdLayer::new(
            HeaderName::from_static("x-request-id"),
            MakeRequestUuid,
        ))
        .layer(CorsLayer::permissive());

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    let _ = axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await;

    Ok(())
}

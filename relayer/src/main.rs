mod signer;

use axum::{routing::post, Router};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let app = Router::new()
        .route("/relayer", post(signer::relay_withdraw))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Relayer listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

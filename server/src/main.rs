use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

mod handler;

#[tokio::main]
async fn main() {
    // Your API router (e.g., /api/data)
    let api = Router::new()
        .route("/data", get(handler::get_data));

    // Serve frontend (e.g., React build output from Vite/CRA)
    let frontend = ServeDir::new("../front-end/my-app/dist")
        .not_found_service(ServeDir::new("../front-end/my-app/dist")); // fallback for SPA routing

    // Final app
    let app = Router::new()
        .nest("/api", api)     // API on /api/...
        .fallback_service(frontend); // Frontend fallback for all other routes

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

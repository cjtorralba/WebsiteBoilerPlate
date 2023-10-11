use axum::response::Response;
use axum::Router;
use axum::routing::*;
use http::StatusCode;
use hyper::Body;
use sqlx::PgPool;
use tower_http::services::*;

use crate::{handlers, layers};
use crate::handlers::root;

/// File handles all our routes and requests
pub async fn app(/* pool: PgPool */) -> Router {
    //let db = Store::with_pool(pool);

    let (cors_layer, trace_layer) = layers::get_layers();

    // MATCHES EXPLICITLY FROM TOP TO BOTTOM
    Router::new()
        .route("/", get(root))
        .route("/*_", get(handle_404)) // '/*_' will match anything not in our routes above
        //.nest_service("/templates", ServeDir::new("styles.css")) STILL FIGURING OUT
        .layer(cors_layer)
        .layer(trace_layer)
        //.with_state(db)
}

async fn handle_404() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("404 not found"))
        .unwrap()
}
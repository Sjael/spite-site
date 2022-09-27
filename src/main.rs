use axum::{response::Html, routing::get, Router, handler::Handler, http::StatusCode};
use std::net::SocketAddr;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .fallback(fallback.into_service())
        .route("/demo.html", get(handler))
        .route("/demo-status", get(demo_status))
        .route("/demo-uri", get(demo_uri))
        .route("/demo.png", get(get_demo_png))
        .route("/foo",
    get(get_foo)
            .put(put_foo)
            .patch(patch_foo)
            .post(post_foo)
            .delete(delete_foo),
        )
        .route("/items/:id",
            get(get_items_id)
        )
        .route("/items",
        get(get_items)
        );

    // run it
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(signal_shutdown())
        .await
        .unwrap();
}

/// axum handler for "GET /items" which uses `axum::extract::Query`.
/// This extracts query parameters and creates a key-value pair map.
pub async fn get_items(
    axum::extract::Query(params):
        axum::extract::Query<HashMap<String, String>>
) -> String {
    format!("Get items with query params: {:?}", params)
}

pub async fn get_items_id(
    axum::extract::Path(id):
        axum::extract::Path<String>
) -> String {
    format!("Get items with path id: {:?}", id)
}

/// axum handler for "GET /foo" which returns a string message.
/// This shows our naming convention for HTTP GET handlers.
pub async fn get_foo() -> String {
    "GET foo".to_string()
 }
 
 /// axum handler for "PUT /foo" which returns a string message.
 /// This shows our naming convention for HTTP PUT handlers.
 pub async fn put_foo() -> String {
    "PUT foo".to_string()
 }
 
 /// axum handler for "PATCH /foo" which returns a string message.
 /// This shows our naming convention for HTTP PATCH handlers.
 pub async fn patch_foo() -> String {
    "PATCH foo".to_string()
 }
 
 /// axum handler for "POST /foo" which returns a string message.
 /// This shows our naming convention for HTTP POST handlers.
 pub async fn post_foo() -> String {
    "POST foo".to_string()
 }
 
 /// axum handler for "DELETE /foo" which returns a string message.
 /// This shows our naming convention for HTTP DELETE handlers.
 pub async fn delete_foo() -> String {
    "DELETE foo".to_string()
 }

async fn handler() -> Html<&'static str> {
    include_str!("spite.html").into()
}

pub async fn demo_status() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Everything is OK".to_string())
}

pub async fn demo_uri(uri: axum::http::Uri) -> String {
    format!("The URI is: {:?}", uri)
}

async fn get_demo_png() -> impl axum::response::IntoResponse {
    let png = concat!(
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB",
        "CAYAAAAfFcSJAAAADUlEQVR42mPk+89Q",
        "DwADvgGOSHzRgAAAAABJRU5ErkJggg=="
    );
    (
        axum::response::AppendHeaders([
            (axum::http::header::CONTENT_TYPE, "image/png"),
        ]),
        base64::decode(png).unwrap(),
    )
}

pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri))
}

async fn signal_shutdown() {
    tokio::signal::ctrl_c()
        .await
        .expect("expect tokio signal ctrl-c");
    println!("signal shutdown");
}
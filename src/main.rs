#![allow(unused)]   // Fo beginning only.

use std::fmt::format;
use std::net::SocketAddr;
use axum::{Json, Router, Server, routing::{get, get_service}, response::{Html, IntoResponse, Response}, extract::{Path, Query}, middleware};
use serde::Deserialize;
use tower_http::services::ServeDir;


mod error;

// pub use self::error::{Error, Result};
pub use self::error::{Error};

mod web;


#[tokio::main]
async fn main() {
    let route_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .fallback_service(route_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server started, listening on {addr}");

    Server::bind(&addr)
        .serve(route_all.into_make_service())
        .await
        .expect("Failed to start server");
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn route_static() -> Router {
    Router::new().nest_service("/src/", get_service(ServeDir::new("./src/")))
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello-json", get(handler_json))
        .route("/hello-html", get(handler_html))
        .route("/hello-html-param", get(handler_html_param))
        .route("/hello-html/:name", get(handler_html_name))
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler_json() -> Json<Message> {
    println!("[localhost:3030] get: /hello-json");

    Json(Message {
        message: String::from("hello, World!"),
    })
}

// async fn handler_html() -> Html<&'static str> {
async fn handler_html() -> impl IntoResponse {
    println!("[localhost:3030] get: /hello-html");

    Html("Hello <strong>World!</strong>")
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// e.g., `/hello-html-param?name=Juice`
async fn handler_html_param(Query(params): Query<HelloParams>) -> impl IntoResponse {
// async fn handler_html_param(params: Query<HelloParams>) -> impl IntoResponse {
    println!("[localhost:3030] get: /hello-html-param");
    println!("--> {:<12} - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");

    Html(format!("Hello <strong>{name}</strong>"))
}

// e.g., `/hello-html/name`
async fn handler_html_name(Path(name): Path<String>) -> impl IntoResponse {
// async fn handler_html_name(name: Path<String>) -> impl IntoResponse {
    println!("[localhost:3030] get: /hello-html/name");
    println!("--> {:<12} - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))
}
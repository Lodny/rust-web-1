#![allow(unused)]   // Fo beginning only.

use std::fmt::format;
use std::net::SocketAddr;

use axum::{extract::{Path, Query}, Json, middleware, response::{Html, IntoResponse, Response}, Router, routing::{get, get_service}, Server};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use crate::ctx::Ctx;
use crate::model::ModelController;

pub use self::error::{Error, Result};

mod ctx;
mod error;
mod model;
mod web;


#[tokio::main]
async fn main() -> Result<()> {
    // Initialize ModelController.
    let mc = ModelController::new().await?;

    let routes_apis = web::routes_tickets::routes(mc.clone())
        .route_layer(middleware::from_fn(web::mw_auth::mw_require_auth));

    let route_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", routes_apis)
        .layer(middleware::map_response(main_response_mapper))
        .layer(middleware::from_fn_with_state(
            mc.clone(),
            web::mw_auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .fallback_service(route_static());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    println!("Server started, listening on {addr}");

    Server::bind(&addr)
        .serve(route_all.into_make_service())
        .await
        .expect("Failed to start server");

    Ok(())
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

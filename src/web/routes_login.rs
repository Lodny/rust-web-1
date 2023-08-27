// use crate::{Error, Result};
use crate::{Error};
use axum::{Json, Router};
use axum::routing::post;
use serde::Deserialize;
use serde_json::{json, Value};


pub type Result<T> = core::result::Result<T, Error>;


pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login", "HANDLER");

    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // TODO: Set Cookies

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}

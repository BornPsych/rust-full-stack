#![allow(unused)]

use axum::{
    extract::{Path, Query}, middleware, response::{IntoResponse, Response}, routing::{get, get_service}, Router
};
use serde::Deserialize;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

pub use self::error::{Error, Result};
mod error;
mod web;

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(route_hello())
        .merge(web::routes_login::routes())
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    println!("Running on the server with port 8000");
    axum::serve(listener, routes_all.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

fn route_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{user_id}", get(handler_hello2))
}

async fn main_response_mapper(res: Response) -> Response {
    println!("main response mapper");
    res
}

fn routes_static() -> Router {
    Router::new().fallback_service(get_service(ServeDir::new("./")))
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    let name = params.name.as_deref().unwrap();
    let response = format!("this is hello sting with params {}", name).into_response();
    println!("this is from Handler function {params:?}");
    return response;
}

async fn handler_hello2(Path(user_id): Path<String>) -> impl IntoResponse {
    let response = format!("this is hello sting with path {user_id}").into_response();
    println!("this is from Handler function {user_id}");
    return response;
}

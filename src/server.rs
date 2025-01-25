use axum::{
    body,
    extract::Query,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};

use crate::graph::LatLon;

pub async fn server_start(address : &str) {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/nav", get(nav));

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    return "Ok"; 
}

#[derive(Debug, Deserialize)]
struct NavParameters {
    orig: String,
    dest: String,
}

#[derive(Debug, Serialize)]
struct NavResponse {
    distance: f64,
    duration: f64,
    path: Vec<LatLon>,
}

impl IntoResponse for NavResponse {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self); // Result<String>
        match body {
            Ok(b) => (StatusCode::OK, b).into_response(), // http 200
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, "json seriliaze error").into_response(), // http 500 error
        }
    }
}

#[derive(Debug, Serialize)]
struct ErrResponse {
    error_message: String,
    code: u64,
}

impl IntoResponse for ErrResponse {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self); // Result<String>
        match body {
            Ok(b) => (StatusCode::OK, b).into_response(), // http 500
            Err(e) => (StatusCode::OK, "json seriliaze error").into_response(), // http 500 error
        }
    }
}

async fn nav(Query(req): Query<NavParameters>) -> Result<NavResponse, ErrResponse> {
    let origin = LatLon::parse(&req.orig).map_err(|e| ErrResponse {
        error_message: "origin  format error".to_string(),
        code: 111,
    })?;

    let destination = LatLon::parse(&req.dest).map_err(|e| ErrResponse {
        error_message: "destination format error".to_string(),
        code: 111,
    })?;

    Ok(NavResponse {
        distance: 0.0,
        duration: 0.0,
        path: vec![origin, destination],
    })
}

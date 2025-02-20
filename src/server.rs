use std::{rc::Rc, sync::Arc};

use axum::{
    body,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use rstar::RTree;
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::{cors::{Any, CorsLayer}, services::{ServeDir, ServeFile}};

use crate::{
    engine::Engine,
    graph::{shortest_path, Graph, LatLon},
    parser::{parse_map, NodeLocation},
};

struct AppState {
    engine: Engine,
}

pub async fn server_start(address: &str, map_path: &str) {
    let engine = Engine::build(map_path).unwrap();
    let shared_state = Arc::new(AppState { engine });

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/nav", get(nav))
        .layer(ServiceBuilder::new().layer(cors))
        // put web/index.html in the same dir of our final binary
        // in docker:
        //    /app (dir)
        //    /app/simple-nav (binary)
        //    /app/web/index.html (static web page)
        .with_state(shared_state);

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

// Rust doesn't have reflect mechanism: runtime type inspection , GOlang's json::marshal() , e.g. mapping struct field name to json keys
// extractor (axum): special trick.
async fn nav(
    Query(req): Query<NavParameters>,
    app_state: State<Arc<AppState>>,
) -> Result<NavResponse, ErrResponse> {
    let origin = LatLon::parse(&req.orig).map_err(|e| ErrResponse {
        error_message: "origin  format error".to_string(),
        code: 111,
    })?;

    let destination = LatLon::parse(&req.dest).map_err(|e| ErrResponse {
        error_message: "destination format error".to_string(),
        code: 111,
    })?;

    let result = app_state.engine.routing(origin, destination).unwrap();

    Ok(NavResponse {
        distance: result.total_distance,
        duration: 0.0,
        path: result.route_path,
    })
}

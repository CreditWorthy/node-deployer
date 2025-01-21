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
    return "Ok"; // string literal, it's in the data section of binary.
}
// serde serialize/deserialize

// Result enum: Err(E) or Ok(T)
// Option enum: None or Some(T)

// Result<f64, _> type inference
// result for reporting error

// expected `LatLon`, found `()`
// String vs &str vs &String

// golang: return (LatLon, error)
// &'static str is usually only for compilie-time known string content. (in binary)

// {"lat": 0.1, "lon": 0.1}
// 0.1,0.1

#[derive(Debug, Deserialize)]
struct NavParameters {
    orig: String,
    dest: String,
}

// "/nav?origin=<lat,lon>&destination=<lat,lon>" get, query parameter

// axum: extractor
// pattern matching syntax: destructoring
// irrefutable (won't fail)

// latlon
// { distance: 100.0, duration: 30.0, path: [[103.1,10.1], [103.2, 10.2]]}
// draw this data on the webmap using tools like mapbox-gl

// serde is general purpose serlization/deserilaizion , not only for json
#[derive(Debug, Serialize)]
struct NavResponse {
    distance: f64,
    duration: f64,
    path: Vec<LatLon>,
}

// it's not suitable here for nav handler error.
impl IntoResponse for NavResponse {
    fn into_response(self) -> axum::response::Response {
        let body = serde_json::to_string(&self); // Result<String>
        match body {
            Ok(b) => (StatusCode::OK, b).into_response(), // http 200
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, "json seriliaze error").into_response(), // http 500 error
        }
    }
}

// axum framework: serialize response

// hetergonous data structure for http response

// {distance: .., duration ..}
// {error_message: "...." }

// {status: "ok" or "error", error_message: "...", data: {distance: .., duration: .. }} // common structure for the response

// String = (Status::OK, String)

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

// mix of rust's trait system and axum itself quirks.
// axum utilize many "black-magic" underlying

// Result<T, E> : T is for Ok(T) variant, E is for Err(E) variant

// in rust: pattern matching is everywhere
async fn nav(Query(req): Query<NavParameters>) -> Result<NavResponse, ErrResponse> {
    let origin = LatLon::parse(&req.orig).map_err(|e| ErrResponse {
        error_message: "origin  format error".to_string(),
        code: 111,
    })?; // String as Err variant

    let destination = LatLon::parse(&req.dest).map_err(|e| ErrResponse {
        error_message: "destination format error".to_string(),
        code: 111,
    })?;
    // ?

    Ok(NavResponse {
        distance: 0.0,
        duration: 0.0,
        path: vec![origin, destination],
    })

    // todo!()

    // match (origin, destination) {
    //     (Ok(origin), Ok(destination)) => {
    //         let resp =
    //         NavResponse{
    //                         distance: 0.0,
    //                         duration: 0.0,
    //                         path: vec![origin, destination]
    //                     };

    //                     // String: json formatted by manually
    //              Ok(resp)
    //     },
    //     _ => {
    //         Err(ErrResponse{error_message: "origin or destination format error".to_string(), code: 111})
    //     }
    //     // (Ok(_), Err(e)) => {Err(ErrResponse{error_message: "destination format error".to_string(), code: 111})},
    //     // (Err(_), Ok(e)) => {Err(ErrResponse{error_message: "origin format error".to_string(), code: 111})},
    //     // (Err(_), Err(e)) => {Err(ErrResponse{error_message: "origin and destination both format error".to_string(), code: 111})},
    //     // Err(e) => Err(ErrResponse{error_message: "orig format error".to_string(), code: 111})
    // }

    // (Status::OK, format!("hello"))

    // match origin {
    //     Ok(origin) => {
    //         Ok(NavResponse{
    //             distance: 0.0,
    //             duration: 0.0,
    //             path: vec![origin, destination.unwrap()]
    //         })
    //     },
    //     Err(e) => return Err("ss".to_string())
    // }
}

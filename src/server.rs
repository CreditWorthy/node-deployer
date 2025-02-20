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
use tower_http::services::{ServeDir, ServeFile};

use crate::{
    engine::Engine,
    graph::{shortest_path, Graph, LatLon},
    parser::{parse_map, NodeLocation},
};

struct AppState {
    engine: Engine,
}

#[test]
fn test_rc() {
    struct S {
        a: f64,
    };

    //  impl !Send for S {}

    // impl Drop for S {
    //     fn drop(&mut self) {
    //         println!("dropping S");
    //     }
    // }

    // {
    //     let s = S{a: 111.0}; // on stack
    //     let mul_owned = Arc::new(s); // T (move by default unless T is copy) vs. &T

    //     // let b = Box::new(s); // allocate `s` on the heap instead of original statck.

    //     // b is on stack. but underlying `s` is on the heap.
    //     // Box / String: owned pointer (in some sense)

    //     // Deref trait: smart pointer in rust
    //     // deref(&self) -> &T:
    //     // mul_owned.a; // mul_owned.deref().a; automically by compiler

    //     // let x = Rc::clone(&mul_owned); // only clone pointer Rc
    //     // let y = mul_owned.clone(); // only clone pointer Rc

    //     // rust prevet misuse of Rc through the Send trait.
    //     // Sync trait: safe borrow to some values in multiple threads.
    //     // &T: Send == T: Sync.
    //     // `S` is Send

    //     let mul_owned2 = Arc::clone(&mul_owned);

    //     //  Arc: immutable share
    //     // Mutex. can give mutable share to its wrapped inner value.

    //     // DerefMut.
    //     let mx = std::sync::Mutex::new(S{a:2.0});

    //     std::thread::spawn(move || {
    //         // child access variable owned by parent: danling pointer
    //         println!("{}", mul_owned.a);
    //         mx.lock().unwrap().a = 2.0;
    //     });

    //     std::thread::spawn(move || {
    //         // child access variable owned by parent: danling pointer
    //         println!("{}", mul_owned2.a);
    //     });

    //     // Rc: not multi-thread safe. how compiler ensure programms not mis-use it in multi-thread env?
    //     // Send & Sync.
    //     // Arc: it's safe version of Rc
    // }

    println!("== s scope finish");
}

pub async fn server_start(address: &str, map_path: &str) {
    let engine = Engine::build(map_path).unwrap();
    let shared_state = Arc::new(AppState { engine });

    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/nav", get(nav))
        // put web/index.html in the same dir of our final binary
        // in docker:
        //    /app (dir)
        //    /app/simple-nav (binary)
        //    /app/web/index.html (static web page)
        .nest_service(
            "/static",
            ServeDir::new("web").not_found_service(ServeFile::new("web/index.html")),
        )
        .with_state(shared_state);

    // 1. ownership system: (RAII) manage memory/resources semi-auto (semi-gc).
    // 2. borrow & references:
    // 3. multiple ownership problem: (only semantic/conceptually) , rust used some special types : Rc / Arc

    // reasoning like:
    // 1. use Mutex to protece shared data in multiple threads
    // 2. and use Mutex::lock() (returns MutexGuard) to aquire lock
    // 3. Drop for MutexGuard to release lock automatically
    // 4. Problem: MutexGuard can not send to another threads.

    // Send: move / transfer ownership to other threads

    // CANNOT DO:
    // have some shared object (shared_state)
    // Mutex wrap (shared_state)
    // MutextGuard cannot be send to multiple threads to access

    // let mutex_shared = std::sync::Mutex::new(String::from("hello")); // not 'static variable

    // *guard = String::from("world");
    // !Send meaning

    // compiler will derive a FnOnce trait implementation for closure here auto.
    // Fn/FnMut (compiler built-in trait)
    // ~~Marker traits examples.~~

    // spawn a child thread in main thread. child thread can run longer the main thread.
    // spawn expects a static lifetime closure parameter
    // std::thread::spawn(move || {
    //     // so if you access some value inside a child thread, memory issue: resource realsed by main thread.
    //     // time sleep 1 hours, mutex_shared destroied/reclaimed because finish of main thread.
    //     // dangling reference/memory

    //     // 'static constraints means closure either owns used variables/values or only refer/borrow some 'static variables/values
    //     let mut guard = mutex_shared.lock().unwrap();
    //     *guard = String::from("world");
    // });

    // Mutex vs Arc.
    // Arc: atomic reference count , 1. multiple ownerships, 2. thread-safe
    // 1. multiple ownerships (conceptually, not syntax lly): e.g. share part of lists
    //     a -> b -> c
    //          ^
    //     d -> e
    //  a and e both owns (b -> c) child list
    //  when releasing list start with `a`, you cannot just release (b -> c) child list.
    //   `Rc` for single thread, `Arc for multi-thread.

    //  Rc<Node>: b
    //  a.

    //

    // Drop trait it's called by compiler when out of scope (RAII):
    // (MutexGuard): it will release the corresponding lock automatically.

    // u64, is Send.
    // lock is usually not safe to send. POSIX
    // lock is not implemented Send trait

    // Send & Sync marker traits (auto trait)
    // Send: express if value of some type can be sent to another thread
    // Sync: express if reference of some type can be sent to another thread.
    // by default compiler will derive these two traits for your defined types,
    // if you want implements Send and Sync you need to use unsafe.

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

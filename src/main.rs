use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health_check", get(health_check));   

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

}

async fn health_check() -> &'static str {
    return "Ok"; // string literal, it's in the data section of binary.
}

struct LatLon {
    lat : f64,
    long : f64,
}

enum E{
    A(i32), // variants
    B(i32)
}


fn match_e(e: E, e2: Json<E>, o1: Option<i32>, /* imagine: o1: Integer */) {
    // let E::A(x) = e;
    // let y:Json<E> = e2;

    // let o1 = Some(5);

    // let o1 = Integer32(555); // you're expecting in java
    // o1.ssss// null pointer exception.

    // idea: using type system to make program safer.
    
    // compiler enforce programmer to check
    // 2nd use case:
    match o1 {
        None=> {
            // cann't make mistake to access some non-exsit data.
        },
        Some(o1_inner) => {
            // you're guranteed that existence of data.
            println!("{}", o1_inner)},
    }

    // compiler gurantted
    if let Some(v) = o1 {

        // block
        println!("{}", v);
    } else {
        
    }

    

    // let v =vec![1,3,3]; // vector / slice
    // match v.as_slice() {
    //     [a, b] => {
    //                    // 4th element out of index range. 
    //     }, // 1. assert two elements, 2. extract the first two elements in slice
    //     _ => {}
    // }
}

// axum: extractor
// pattern matching syntax: destructoring
// irrefutable (won't fail)
async fn nav(Json(payload): Json<(LatLon, LatLon)>) {
    let first_latlon = payload.0; // syntax to access tuple element
    let second_latlon = payload.1; // syntax to access tuple element
    
}
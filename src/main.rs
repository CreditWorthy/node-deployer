mod server;
mod parser;
mod graph;

// tokio::main expects main() returns nothing -> ()
#[tokio::main]
async fn main() {
    
    // server::server_start("0.0.0.0:3000") 

    // Future: related async
    server::server_start("0.0.0.0:3000").await // constructed a future object
    // x.await; // future object is lazy, it's not executed.

}
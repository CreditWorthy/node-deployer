use simple_nav::server;

// tokio::main expects main() returns nothing -> ()
#[tokio::main]
async fn main() {

    let args:Vec<_> = std::env::args().collect();
    
    if args.len() < 2 {
        println!("need map pbf parameter");
        return;
    }

    // args: Vec<String>
    let map_pbf = &args[1]; // the second positional parameter when running our program
    
    // server::server_start("0.0.0.0:3000") 

    // Future: related async
    server::server_start("0.0.0.0:3000", map_pbf).await // constructed a future object
    // x.await; // future object is lazy, it's not executed.
    
}
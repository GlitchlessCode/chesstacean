use warp::{http::Response, Filter};

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let first = warp::path("game").and(warp::path::end()).map(|| "Hello!");
    let second = warp::any().map(|| {
        Response::builder().status(404).body(
            "<!DOCTYPE html><html><head></head><body><h1>404 Page Not Found</h1></body></html>",
        )
    });
    let test = first.or(second);

    warp::serve(test).run(([127, 0, 0, 1], 3000)).await;
}

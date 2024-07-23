use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::get().map(|| format!("Hello\n"));

    warp::serve(hello).run(([127, 0, 0, 1], 6969)).await;
}

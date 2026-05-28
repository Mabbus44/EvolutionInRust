mod simulation;
mod json_helper;
mod web_server;

use web_server::run;

#[tokio::main]
async fn main() {
    run().await;
}

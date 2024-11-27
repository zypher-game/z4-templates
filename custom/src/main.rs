mod handler;

use z4_engine::{Config, Engine};
// use z4_pozk::Engine; // if you want to build on PoZK, use this Engine
use handler::GameHandler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // use Z4 engine to build self-host node
    let config = Config::from_env().unwrap();
    Engine::<GameHandler>::init(config).run().await.expect("Down");

    // use PoZK engine to build a PoZK prover
    // z4_pozk::Engine::<GameHandler>::run().await.expect("Down");
}

mod handler;
use handler::GameHandler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // use Z4 engine to build self-host node
    let config = z4_engine::Config::from_env().unwrap();
    z4_engine::Engine::<GameHandler>::init(config).run().await.expect("Down");

    // use PoZK engine to build a PoZK prover, it need INPUT=xxx, it will supported by PoZK miner
    // z4_pozk::Engine::<GameHandler>::run().await.expect("Down");
}

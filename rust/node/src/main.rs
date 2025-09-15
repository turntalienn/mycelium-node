use clap::Parser;
#[derive(Parser)]
struct Args{
    #[arg(short, long, default_value_t = 4001)]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    env_logger::init();
    let args = Args.parse();
    log::info!("Starting Mycelium node on port {}", args.port);

    println!("hello mycelium listening on {}", args.port);

    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}
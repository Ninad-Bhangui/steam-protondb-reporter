use protondb_scanner::config::Config;
use std::{env, process};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing config: {}", err);
        process::exit(1);
    });
    protondb_scanner::run(config).await?;
    Ok(())
}

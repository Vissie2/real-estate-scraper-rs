mod cli;
mod config;

#[tokio::main]
async fn main() {
    println!("Hi.");
    let config = config::get_config();

    cli::initialize(config);
}

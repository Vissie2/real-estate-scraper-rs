use clap::Parser;

#[derive(Debug)]
pub struct Config {
    pub min_price: i32,
    pub max_price: i32,
    pub place: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(
        long,
        required(true),
        value_delimiter = ',',
        help("Two integers separated by a comma.")
    )]
    pub price_range: Vec<i32>,

    #[arg(long, required(true))]
    pub place: String,
}

pub fn get_config() -> Config {
    let args = Args::parse();

    let (min_price, max_price) = match args.price_range[0..2] {
        [x, y] => (x, y),
        _ => panic!("Please provide a valid price range."),
    };

    Config {
        min_price,
        max_price,
        place: args.place,
    }
}

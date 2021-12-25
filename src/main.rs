use crypto_get::get_coins;
use crypto_print::format_coins;
use env_logger;
use log::debug;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    coins: Vec<String>,
}


fn main() {
    env_logger::init();
    debug!("Initialized logger");

    let args = Cli::from_args();

    let coins = get_coins(args.coins);
    format_coins(coins.unwrap())
}

use prettytable::{Table, Row, Cell};
use std::ops::Add;
use thousands::Separable;
use std::vec;
pub use crate::get::Coin;

fn format_multiplier(base_market_cap: f64, coin_market_cap: f64, coin_price: f64) -> String {
    let multiplier = (base_market_cap / coin_market_cap).round();
    let slash = " / ";
    let multiplied_price = (coin_price * multiplier).round();
    multiplier.to_string().add(slash).add(&multiplied_price.to_string())
}

fn format_24_hour_change(coin: &Coin) -> String {
    let slash = " / ";
    let percentage = "%";
    coin.price_change_24h.to_string().add(slash).add(&coin.price_change_percentage_24h.to_string()).add(percentage)
}

fn unpack_option_32(option: Option<f32>) -> String {
    let result = match option {
        Some(inner) => inner,
        None => 0.0
    };

    result.to_string()
}

fn unpack_option_64(option: Option<f64>) -> String {
    let result = match option {
        Some(inner) => inner,
        None => 0.0
    };

    result.to_string()
}

pub fn format_coins(coins: Vec<Coin>) {
    let mut table = Table::new();

    table.add_row(row![
        "",
        "PRICE",
        "RANK",
        "24 HOUR PRICE CHANGE",
        "CIRCULATING SUPPLY",
        "TOTAL SUPPLY",
        "MARKET CAP",
        "MULTIPLIER/PRICE TO EQUAL BTC",
        "MULTIPLIER/PRICE TO EQUAL ETH",
    ]);

    let btc = coins.get(0);
    let btc_market_cap = btc.unwrap().market_cap;

    let eth = coins.get(1);
    let eth_market_cap = eth.unwrap().market_cap;

    for (i, coin) in coins.iter().enumerate() {
        table.add_row(Row::new(vec![
            Cell::new(&coin.name),
            Cell::new(&coin.current_price.separate_with_commas()),
            Cell::new(&unpack_option_32(coin.market_cap_rank)),
            Cell::new(
                &format_24_hour_change(&coin)
            ),
            Cell::new(&coin.circulating_supply.separate_with_commas()),
            Cell::new(&unpack_option_64(coin.total_supply).separate_with_commas()),
            Cell::new(&coin.market_cap.separate_with_commas()),
            if i == 0 { Cell::new("") } else {
                Cell::new(
                    &format_multiplier(btc_market_cap, coin.market_cap, coin.current_price).to_string()
                )
            },
            if i == 0 || i == 1 { Cell::new("")} else {
                Cell::new(
                    &format_multiplier(eth_market_cap, coin.market_cap, coin.current_price).to_string()
                )
            },
        ]));
    }


    table.printstd();
}
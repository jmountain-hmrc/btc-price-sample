#[macro_use]
extern crate serde_json;
extern crate reqwest;

use std::io;
use serde_json::Value;

fn perform_request(cur_type: Option<&String>) -> Result<(), reqwest::Error> {
    let mut res  = reqwest::get("https://api.coindesk.com/v1/bpi/currentprice.json")?;
    let body = res.text()?;

    let parsed_json : Value = serde_json::from_str(&body).unwrap();

    let mut currencies = vec!["GBP", "USD", "EUR"];
    if let Some(currency) = cur_type {
        currencies = vec![currency.trim()];
    }

    for currency in &currencies {
        let parsed_currency = &parsed_json["bpi"][currency];

        if parsed_currency.is_null() {
            println!("Currency {} does not exist!", currency);
        } else {
            let currency_name = &parsed_currency["description"].as_str().unwrap();

            println!("{} - {} -> BTC: {} -> 1 BTC",
                currency_name, currency, &parsed_currency["rate"].as_str().unwrap());
        }
    }

    Ok(())
}

fn main() {
    let mut currency = String::new();

    println!("Enter a currency, or omit to show a basic set of them");

    let res = io::stdin()
        .read_line(&mut currency)
        .ok()
        .map(|_| &currency)
        .filter(|str| str.trim().len() > 0);

    perform_request(res);
}
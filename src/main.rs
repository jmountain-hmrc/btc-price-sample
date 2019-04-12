#[macro_use]
extern crate serde_json;
extern crate reqwest;
extern crate mongodb;

use std::io;
use serde_json::Value;
use mongodb::{Bson, bson, doc};
use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;

fn get_collection() -> Collection {
    let client = Client::connect("localhost", 27017)
        .expect("Could not connect to Mongo");

    client.db("rust").collection("currencies")
}

fn save_to_mongo(currency_name: &String, rate: &String) {
    get_collection().insert_one(doc!{
        "currency_name" : currency_name,
        "rate" : rate
    }, None).unwrap();
}

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
            let currency_rate = &parsed_currency["rate"].as_str().unwrap();

            println!("{} - {} -> BTC: {} -> 1 BTC", currency_name, currency, currency_rate);

            let mongo_save_name = currency.to_string();
            let mongo_save_rate = currency_rate.to_string();
            save_to_mongo(&mongo_save_name, &mongo_save_rate);
        }
    }

    Ok(())
}

fn list_previous_results() {
    let mut cursor = get_collection().find(Some(doc!{}), None)
        .ok().expect("Could not retrieve documents from mongo");
        
    match cursor.next() {
        Some(Ok(doc))   => {
            match (doc.get("currency_name"), doc.get("rate")) {
                (Some(&Bson::String(ref name)), Some(&Bson::String(ref rate))) => println!("{} - {}", name, rate),
                _ => panic!("!!!")
            }
        }
        Some(Err(_))    => panic!("!!!"),
        None            => panic!("!!!")
    }
}

fn main() {
    let mut currency = String::new();

    println!("Enter a currency, enter `previous` to show previous results, or omit to show a basic set of them");

    let res = io::stdin()
        .read_line(&mut currency)
        .ok()
        .map(|_| &currency)
        .filter(|str| str.trim().len() > 0);

    match res {
        Some(input) if input.trim() == "previous" => list_previous_results(),
        anything_else    => {
            perform_request(res);
        }
    }
}
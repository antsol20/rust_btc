use chrono::{NaiveDate, Datelike, Weekday};
use reqwest;
use serde_json;
use serde_json::Value;
use std::io::Read;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3{
        println!("Usage: rust_btc [start-date] [end-date]   note: dates are in format YYYY-MM-DD");
        //return;
    }
    let start_date_text = &args[1];
    let end_date_text = &args[2];
    
    let suffix = "T00:00:00Z";

    let start = start_date_text.to_owned() + suffix;
    let end = end_date_text.to_owned() + suffix;

    println!("BTC price analysis in Rust");
    
    println!("start is {}", start);
    println!("end is {}", end);
    
    let api_key = std::fs::read_to_string("apikey.txt").unwrap();
    let request_url = format!("https://api.nomics.com/v1/exchange-rates/history?key={api_key}&currency=BTC&start={start}&end={end}",
                              api_key = api_key, start=start, end=end);


    let resp = reqwest::blocking::get(request_url);
    let mut body = String::new();

    match resp{
        Ok(mut resp_ok) => {
            resp_ok.read_to_string(&mut body).unwrap();
        }
        Err(_) => {
            println!("Error with data pull. Exiting");
            return;
        }
    }

    let v: Value = serde_json::from_str(&body).unwrap();
    let json = v.as_array().unwrap();

    let date_fmt = "%Y-%m-%d";


    let mut price_pairs:Vec<(f32,f32)> = vec![];
    let mut price_pair:(f32,f32) = (0.0, 0.0);

    for entry in json  {

        let datestamp = &entry["timestamp"].as_str().unwrap()[..10];
        let week_day = NaiveDate::parse_from_str(datestamp, date_fmt).unwrap().weekday();
        let btc = entry["rate"].as_str().unwrap().parse::<f32>().unwrap();

        if week_day == Weekday::Fri{
            price_pair.0 = btc;
        }

        if week_day == Weekday::Mon{
            if price_pair.0 == 0.0{
                continue;
            }
            price_pair.1 = btc;
            price_pairs.push(price_pair);
        }

    }

    let mut cash: f32 = 100.0;

    for prices in price_pairs{
        println!("Start Cash {} Buy:{} Sell:{}", cash, prices.0, prices.1);
        cash = cash * (prices.1/prices.0);
        println!("End Cash {}", cash)
    }



}

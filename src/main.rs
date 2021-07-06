use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 3{
        println!("Usage: rust_btc [start-date] [end-date]   note: dates are in format YYYYMMDD");
        return;
    }
    let start_date_text = &args[1];
    let end_date_text = &args[2];

    let fmt_in = "%Y%m%d";
    let fmt = "%Y:%m:%d";

    let start_date = NaiveDate::parse_from_str("start_date_text", fmt_in).unwrap();
    let end_date = NaiveDate::parse_from_str("end_date_text", fmt_in).unwrap();

    let start = start_date.format(fmt).to_string();
    let end = start_date.format(fmt).to_string();
    // T00%3A00%3A00Z&

    println!("start is {}", start);
    println!("end is {}", end);
    println!("BTC price analysis in Rust");
    // let api_key = std::fs::read_to_string("apikey.txt").unwrap();
    //
    // let request_url = format!("https://api.nomics.com/v1/exchange-rates/history?key={api_key}&currency=BTC&start={start_date}&end={end_date}",
    //                           api_key = "rust-lang-nursery", start_date = "rust-cookbook", end_date = "rust-cookbook");
    //
    //
    // let mut res = reqwest::blocking::get(url);
    // let mut body = String::new();
    // res.read_to_string(&mut body)?;
    //
    // println!("Status: {}", res.status());
    // println!("Headers:\n{:#?}", res.headers());
    // println!("Body:\n{}", body);
    //

}

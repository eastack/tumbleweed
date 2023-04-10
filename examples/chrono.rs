use chrono::{Local, DateTime};

fn main() {
    let now = Local::now();
    let date = DateTime::parse_from_str("2014-11-28 21:00:09 +09:00", "%Y-%m-%d %H:%M:%S %z");
    println!("Current datetime: {now}");
    println!("Parsed datetime: {date:?}");
}

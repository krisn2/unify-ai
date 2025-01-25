use std::env;
use dotenv::dotenv;

fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY must be set");
    println!("{}", api_key);
}

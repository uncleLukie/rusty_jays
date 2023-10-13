extern crate dotenv;

mod discord_presence;
mod song_fetcher;
mod utils;

fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Your existing code here
    println!("Hello, world!");
}

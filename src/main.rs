extern crate dotenv;

// mods
mod discord_presence;
mod song_fetcher;
mod utils;

// uses
use std::error::Error;
use song_fetcher::{WebAutomator, SongFetcher};
use discord_presence::DiscordPresence;
use tokio;
use webdriver_downloader::prelude::*;
use webdriver_downloader::driver_impls::chromedriver_info::ChromedriverInfo;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialise and download chromedriver if it is not installed
    let driver_info = ChromedriverInfo::new_default().unwrap();
    if !driver_info.is_installed().await {
        if let Err(e) = driver_info.download_verify_install(5).await {
            eprintln!("Failed to download and install chromedriver: {}", e);
            return Err(e.into());
        }
    }

    // Initialise WebAutomator
    let web_automator_result = WebAutomator::new().await;
    let mut web_automator = match web_automator_result {
        Ok(web_automator) => web_automator,
        Err(e) => {
            eprintln!("Failed to initialise WebAutomator: {}", e);
            return Err(e.into());
        }
    };

    // Initialise DiscordPresence
    let discord_presence_result = DiscordPresence::new();
    let mut discord_presence = match discord_presence_result {
        Ok(discord_presence) => discord_presence,
        Err(e) => {
            eprintln!("Failed to initialise DiscordPresence: {}", e);
            return Err(e.into());
        }
    };

    // Initialise SongFetcher (replace 'your_url_here' with the actual URL)
    let song_fetcher = SongFetcher::new("https://www.abc.net.au/triplej/live/triplej");

    // Infinite loop to keep updating Discord status
    loop {
        // Fetch song info (title and artist)
        let fetch_result = song_fetcher.fetch_song_info().await;
        let (song_title, song_artist) = match fetch_result {
            Ok((title, artist)) => (title, artist),
            Err(e) => {
                eprintln!("Failed to fetch song info: {}", e);
                return Err(e.into());
            }
        };

        // Update Discord presence with the fetched song and artist information
        if let Err(e) = discord_presence.update_status(&song_title, &song_artist) {
            eprintln!("Failed to update Discord presence: {}", e);
            return Err(e.into());
        }

        // Wait for some time before the next iteration (10 seconds)
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}

extern crate dotenv;

// mods
mod discord_presence;
mod song_fetcher;
mod utils;

// uses
use std::error::Error;
use song_fetcher::{WebAutomator, SongFetcher};
use discord_presence::DiscordPresence;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load environment variables from .env file
    dotenv::dotenv().ok();

    // initialise webautomator and discordpresence
    let mut web_automator = WebAutomator::new().await?;
    let mut discord_presence = DiscordPresence::new()?;

    // initialise songfetcher (replace 'your_url_here' with the actual url)
    let song_fetcher = SongFetcher::new("https://www.abc.net.au/triplej/live/triplej");

    // infiniloop to keep updating da discord status
    loop {
        // fetch song info (title and artist)
        let (song_title, song_artist) = song_fetcher.fetch_song_info().await?;

        // update discord presence with the fetched song and artist information
        discord_presence.update_status(&song_title, &song_artist)?;

        // wait for some time before the next iteration (10 seconds)
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}

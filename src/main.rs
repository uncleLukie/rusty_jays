use std::env;
use std::error::Error;
use std::fmt::{self, Display};

// Importing external crates
use tokio;

// Importing internal modules
mod discord_presence;
mod song_fetcher;

use song_fetcher::fetch_current_song; // Update this import
use discord_presence::DiscordPresence;

// Custom error enum
#[derive(Debug)]
enum MyError {
    DiscordError(discord_presence::DiscordPresenceError),
    ReqwestError(reqwest::Error),
}

impl Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::DiscordError(err) => write!(f, "Discord Error: {}", err),
            MyError::ReqwestError(err) => write!(f, "Reqwest Error: {}", err),
        }
    }
}

impl Error for MyError {}

impl From<discord_presence::DiscordPresenceError> for MyError {
    fn from(err: discord_presence::DiscordPresenceError) -> MyError {
        MyError::DiscordError(err)
    }
}

impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> MyError {
        MyError::ReqwestError(err)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialise DiscordPresence
    let discord_presence_result = DiscordPresence::new().map_err(MyError::from);

    let mut discord_presence = match discord_presence_result {
        Ok(dp) => dp,
        Err(e) => {
            eprintln!("Failed to initialise DiscordPresence: {:?}", e);
            return Err(e.into());
        }
    };

    // Infinite loop to keep updating Discord status
    loop {
        // Fetch current song info
        let fetch_result: Result<_, MyError> = fetch_current_song().await.map_err(MyError::from);

        match fetch_result {
            Ok(info) => {
                println!("Song: {}", info.title);
                println!("Artist: {}", info.artist);
                println!("Played Time: {}", info.played_time);
                println!("Album: {}", info.album);

                // Update Discord presence with the fetched song and artist information
                discord_presence.update_status(&info.title, &info.artist).map_err(MyError::from)?;
            }
            Err(e) => {
                eprintln!("Error occurred: {}", e);
                return Err(e.into());
            }
        }

        // Wait for some time before the next iteration (let's say, 10 seconds)
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}

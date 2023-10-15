use std::error::Error;
use std::fmt::{self, Display};

// Importing external crates
use tokio;

// Importing internal modules
mod discord_presence;
mod song_fetcher;

use song_fetcher::fetch_current_song;
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
async fn main() {
    dotenv::dotenv().ok();

    let discord_presence_result = DiscordPresence::new().map_err(MyError::from);
    let mut discord_presence = match discord_presence_result {
        Ok(dp) => dp,
        Err(e) => {
            eprintln!("Failed to initialise DiscordPresence: {:?}", e);
            return;
        },
    };

    loop {
        let fetch_result: Result<_, MyError> = fetch_current_song().await.map_err(MyError::from);

        match fetch_result {
            Ok(info) => {
                let (title, artist, played_time) = (
                    info.title.as_deref().unwrap_or("N/A"),
                    info.artist.as_deref().unwrap_or("N/A"),
                    info.played_time.as_deref().unwrap_or("N/A"),
                );

                println!("Song: {}", title);
                println!("Artist: {}", artist);
                println!("Played Time: {}", played_time);
                println!("Album: {}", info.album.as_deref().unwrap_or("N/A"));

                update_discord_status(&mut discord_presence, title, artist, played_time);
            },
            Err(e) => {
                eprintln!("Error occurred: {}", e);
                update_discord_status(&mut discord_presence, "N/A", "N/A", "N/A");
            },
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}

fn update_discord_status(discord_presence: &mut DiscordPresence, title: &str, artist: &str, played_time: &str) {
    println!("Updating Discord status...");  // Debug print

    if title == "N/A" {
        println!("Title is N/A. Setting to 'Listening to the jays'");  // Debug print
        let test_timestamp = "2023-10-15T01:11:51+00:00"; // Replace this with a valid RFC3339 timestamp a few minutes in the past
        if let Err(e) = discord_presence.update_status("Listening to the jays", "", test_timestamp) {
            eprintln!("Failed to set fallback Discord status: {}", e);
        }
    } else {
        println!("Setting title to '{}', artist to '{}'", title, artist);  // Debug print
        if let Err(e) = discord_presence.update_status(title, artist, played_time) {
            eprintln!("Failed to update Discord status: {}", e);
        }
    }
}


use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient, activity::Button, activity::Timestamps};
use std::env;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

extern crate chrono;
use chrono::prelude::*;

// Custom error type for DiscordPresence
#[derive(Debug)]
pub enum DiscordPresenceError {
    InitializationError(String),
    UpdateStatusError(String),
}

impl Display for DiscordPresenceError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            DiscordPresenceError::InitializationError(e) => write!(f, "Discord Initialization Error: {}", e),
            DiscordPresenceError::UpdateStatusError(e) => write!(f, "Discord Update Status Error: {}", e),
        }
    }
}

impl Error for DiscordPresenceError {}

pub struct DiscordPresence {
    client: DiscordIpcClient,
    app_start_time: i64,  // Unix timestamp for when the app started
}

impl DiscordPresence {
    pub fn new() -> Result<Self, DiscordPresenceError> {
        let app_id_str = env::var("DISCORD_APP_ID").map_err(|_| DiscordPresenceError::InitializationError("DISCORD_APP_ID must be set".into()))?;
        let mut client = DiscordIpcClient::new(&app_id_str).map_err(|_| DiscordPresenceError::InitializationError("Failed to create Discord client".into()))?;
        client.connect().map_err(|_| DiscordPresenceError::InitializationError("Failed to connect to Discord".into()))?;

        let app_start_time = Utc::now().timestamp();  // Record the time the app started

        Ok(Self { client, app_start_time })
    }

    pub fn update_status(&mut self, song_title: &str, song_artist: &str, played_time: &str) -> Result<(), DiscordPresenceError> {
        let unix_timestamp = if played_time == "N/A" {
            self.app_start_time  // Use the time the app started as the fallback
        } else {
            DateTime::parse_from_rfc3339(played_time).map(|dt| dt.timestamp()).map_err(|_| DiscordPresenceError::UpdateStatusError("Invalid played_time format".into()))?
        };

        // Create the Button object
        let button = Button::new("GitHub Repo", "https://github.com/uncleLukie/rusty_jays");

        // Create the Timestamps object
        let timestamps = Timestamps::new().start(unix_timestamp);

        // Fetch the large image key from the environment variables
        let large_image_key = env::var("LARGE_IMAGE_KEY").map_err(|_| DiscordPresenceError::UpdateStatusError("LARGE_IMAGE_KEY must be set".into()))?;

        // Create the Assets object
        let assets = activity::Assets::new().large_image(&large_image_key).large_text("Listening to music");

        let payload = activity::Activity::new()
            .state(song_title)
            .details(song_artist)
            .assets(assets)  // Use the Assets object
            .buttons(vec![button])  // Add the Button object
            .timestamps(timestamps); // Add the Timestamps object

        self.client.set_activity(payload).map_err(|_| DiscordPresenceError::UpdateStatusError("Failed to set activity".into()))?;

        Ok(())
    }
}

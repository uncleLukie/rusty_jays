use discord_rpc_client::Client as DiscordClient;
use std::env;
use std::error::Error;
use std::fmt::{self, Display, Formatter};

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
    client: DiscordClient,
}

impl DiscordPresence {
    pub fn new() -> Result<Self, DiscordPresenceError> {
        let app_id_str = env::var("DISCORD_APP_ID").map_err(|_| DiscordPresenceError::InitializationError("DISCORD_APP_ID must be set".into()))?;
        let app_id = app_id_str.parse::<u64>().map_err(|_| DiscordPresenceError::InitializationError("Failed to parse DISCORD_APP_ID to u64".into()))?;

        let mut client = DiscordClient::new(app_id);
        client.start();

        Ok(Self { client })
    }

    pub fn update_status(&mut self, song_title: &str, song_artist: &str) -> Result<(), DiscordPresenceError> {
        let large_image_key = env::var("LARGE_IMAGE_KEY").map_err(|_| DiscordPresenceError::UpdateStatusError("LARGE_IMAGE_KEY must be set".into()))?;

        self.client
            .set_activity(|act| {
                act.state(song_title)
                    .details(song_artist)
                    .assets(|ass| ass.large_image(&large_image_key).large_text("Listening to music"))
            })
            .map_err(|_| DiscordPresenceError::UpdateStatusError("Failed to set activity".into()))?;

        Ok(())
    }
}

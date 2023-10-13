use discord_rpc_client::Client as DiscordClient;
use std::env;
use std::error::Error;

pub struct DiscordPresence {
    client: DiscordClient,
}

impl DiscordPresence {
    // initialise DeRP hurr durr
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let app_id_str = env::var("DISCORD_APP_ID").expect("DISCORD_APP_ID must be set");
        let app_id = app_id_str.parse::<u64>().expect("Failed to parse DISCORD_APP_ID to u64");

        let mut client = DiscordClient::new(app_id);
        client.start();

        Ok(Self { client })
    }

    // update your stinky discord status hehehhhe
    pub fn update_status(&mut self, song_title: &str, song_artist: &str) -> Result<(), Box<dyn Error>> {
        let large_image_key = env::var("LARGE_IMAGE_KEY").expect("LARGE_IMAGE_KEY must be set");

        self.client
            .set_activity(|act| {
                act.state(song_title)
                    .details(song_artist)
                    .assets(|ass| ass.large_image(&large_image_key).large_text("Listening to music"))
            })
            .expect("Failed to set activity");

        Ok(())
    }
}

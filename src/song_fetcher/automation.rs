use thirtyfour::prelude::*;
use tokio;
// Make sure to import DiscordPresence if it's in another module
use crate::discord_presence::DiscordPresence;  // Update the path according to your project structure

pub struct WebAutomator {
    driver: WebDriver,
}

impl WebAutomator {
    // create a new webautomator instance
    pub async fn new() -> Result<Self, WebDriverError> {
        let caps: Capabilities = DesiredCapabilities::chrome().into();
        let driver = WebDriver::new("http://localhost:4444/wd/hub", caps).await?;

        // set window size like in python code
        driver.execute("window.resizeTo(480, 640);", vec![]).await?;

        Ok(Self { driver })
    }

    // navigate to the target url and keep updating discord status
    pub async fn navigate_and_interact(&self, url: &str, discord_presence: &mut DiscordPresence) -> Result<(), WebDriverError> {
        // go to the url
        self.driver.goto(url).await?;

        // keep the browser running indefinitely
        loop {
            // fetch song and artist information
            let (song_title, song_artist) = self.fetch_song_and_artist().await?;

            // update discord presence with the fetched information
            match discord_presence.update_status(&song_title, &song_artist) {
                Ok(_) => println!("successfully updated discord status."),
                Err(e) => eprintln!("failed to update discord status: {}", e),
            }

            // wait for some time before the next iteration
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    }

    // fetch the song and artist information
    pub async fn fetch_song_and_artist(&self) -> Result<(String, String), WebDriverError> {
        let song_info_element = self
            .driver
            .find(By::Css(".Song_nowSongWrapper__BH7vs"))
            .await?;

        let song_title = song_info_element
            .find(By::Css(".Song_songTitle__2Yq_U"))
            .await?
            .text()
            .await?;

        let song_artist = song_info_element
            .find(By::Css(".Song_artistName__2Xm5f"))
            .await?
            .text()
            .await?;

        Ok((song_title, song_artist))
    }
}

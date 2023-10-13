use thirtyfour::prelude::*;
use tokio;

pub struct WebAutomator {
    driver: WebDriver,
}

impl WebAutomator {
    // create a new webautomator instance
    pub async fn new() -> Result<Self, WebDriverError> {
        let caps = DesiredCapabilities::chrome();
        let driver = WebDriver::new("http://localhost:4444/wd/hub", &caps).await?;

        // set window size like in python code
        driver.set_window_size(480, 640).await?;

        Ok(Self { driver })
    }

    // navigate to the target url and maybe click a button or something
    pub async fn navigate_and_interact(&self, url: &str) -> Result<(), WebDriverError> {
        // go to the url
        self.driver.goto(url).await?;

        // find the element with the song info, using the actual selector
        let song_info = self
            .driver
            .find(By::Css(".Song_nowSongWrapper__BH7vs"))
            .await?;

        // you can add additional interactions with the song_info element here

        Ok(())
    }
}

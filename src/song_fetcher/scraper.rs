use reqwest;
use scraper::{Html, Selector};
use std::error::Error;

pub struct SongFetcher {
    url: String,
}

impl SongFetcher {
    // initialise a new songfetcher instance
    pub fn new(url: &str) -> Self {
        Self { url: url.to_string() }
    }

    // fetch song info (title and artist)
    pub async fn fetch_song_info(&self) -> Result<(String, String), Box<dyn Error>> {
        // make an http get request to fetch the raw html content
        let resp = reqwest::get(&self.url).await?;
        let body = resp.text().await?;

        // parse the html content
        let document = Html::parse_document(&body);

        // use the actual selector for song info
        let song_info_selector = Selector::parse(".Song_nowSongWrapper__BH7vs").unwrap();

        // extract song title and artist
        let song_info = document
            .select(&song_info_selector)
            .next()
            .ok_or("Song info not found")?;

        let song_title_selector = Selector::parse(".Song_songTitle__2Yq_U").unwrap();
        let song_title = song_info
            .select(&song_title_selector)
            .next()
            .ok_or("Song title not found")?
            .inner_html();

        let song_artist_selector = Selector::parse(".Song_artistName__2Xm5f").unwrap();
        let song_artist = song_info
            .select(&song_artist_selector)
            .next()
            .ok_or("Song artist not found")?
            .inner_html();

        Ok((song_title, song_artist))
    }
}

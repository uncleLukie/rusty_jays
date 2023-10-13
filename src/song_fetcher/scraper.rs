use reqwest;
use scraper::{Html, Selector};

pub struct SongFetcher {
    url: String,
}

impl SongFetcher {
    pub fn new(url: &str) -> Self {
        Self { url: url.to_string() }
    }

    pub async fn fetch_song_info(&self) -> Result<(String, String), Box<dyn std::error::Error>> {
        // make an http get request to fetch the raw html content
        let resp = reqwest::get(&self.url).await?;
        let body = resp.text().await?;

        // parse the html content
        let document = Html::parse_document(&body);

        // use the actual selector for song info
        let song_info_selector = Selector::parse(".Song_nowSongWrapper__BH7vs").unwrap();

        // extract song title and artist, and uhh i'm totally approximating here...
        let song_info = document
            .select(&song_info_selector)
            .next()
            .ok_or("Song info not found")?;

        let song_title = song_info
            .select(&Selector::parse(".title").unwrap())
            .next()
            .ok_or("Song title not found")?
            .inner_html();

        let song_artist = song_info
            .select(&Selector::parse(".artist").unwrap())
            .next()
            .ok_or("Song artist not found")?
            .inner_html();

        Ok((song_title, song_artist))
    }
}
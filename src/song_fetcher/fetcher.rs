use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Song {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub played_time: Option<String>,
    pub album: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Artist {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Recording {
    title: String,
    artists: Vec<Artist>,
}

#[derive(Debug, Deserialize)]
struct Releases {
    title: String,
}

// Removed Prev struct

// Updated Now struct
#[derive(Debug, Deserialize)]
struct Now {
    recording: Option<Recording>,
    releases: Option<Vec<Releases>>,
    #[serde(rename = "played_time")]
    played_time: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    now: Now,  // Removed prev field
}

pub async fn fetch_current_song() -> Result<Song, Error> {
    let url = "https://music.abcradio.net.au/api/v1/plays/triplej/now.json?tz=Australia%2FSydney";
    let resp: ApiResponse = reqwest::get(url).await?.json().await?;

    let title = resp.now.recording.as_ref().map(|r| r.title.clone());
    let artist = resp.now.recording.as_ref().and_then(|r| r.artists.get(0)).map(|a| a.name.clone());
    let album = resp.now.releases.as_ref().and_then(|r| r.get(0)).map(|r| r.title.clone());
    let played_time = resp.now.played_time.clone();

    Ok(Song {
        title,
        artist,
        played_time,
        album,
    })
}

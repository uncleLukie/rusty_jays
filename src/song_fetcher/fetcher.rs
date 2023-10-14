use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Song {
    pub title: String,
    pub artist: String,
    pub played_time: String,
    pub album: String,
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

#[derive(Debug, Deserialize)]
struct Prev {
    recording: Recording,
    releases: Option<Vec<Releases>>,
    #[serde(rename = "played_time")]
    played_time: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    prev: Prev,
}

pub async fn fetch_current_song() -> Result<Song, Error> {
    let url = "https://music.abcradio.net.au/api/v1/plays/triplej/now.json?tz=Australia%2FSydney";
    let resp: ApiResponse = reqwest::get(url).await?.json().await?;

    let artist = match resp.prev.recording.artists.get(0) {
        Some(artist_data) => artist_data.name.clone(),
        None => "Unknown Artist".to_string(),
    };

    let album = match &resp.prev.releases {
        Some(releases) => releases.get(0).map_or("Unknown Album".to_string(), |r| r.title.clone()),
        None => "Unknown Album".to_string(),
    };

    Ok(Song {
        title: resp.prev.recording.title,
        artist,
        played_time: resp.prev.played_time,
        album,
    })
}

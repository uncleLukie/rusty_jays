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

// New struct for Now
#[derive(Debug, Deserialize)]
struct Now {
    recording: Option<Recording>,
    releases: Option<Vec<Releases>>,
    #[serde(rename = "played_time")]
    played_time: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    prev: Prev,
    now: Now,  // Added now field
}

pub async fn fetch_current_song() -> Result<Song, Error> {
    let url = "https://music.abcradio.net.au/api/v1/plays/triplej/now.json?tz=Australia%2FSydney";
    let resp: ApiResponse = reqwest::get(url).await?.json().await?;

    let (title, artist, album) = if let Some(recording) = &resp.now.recording {
        let title = recording.title.clone();
        let artist = match recording.artists.get(0) {
            Some(artist_data) => artist_data.name.clone(),
            None => "Unknown Artist".to_string(),
        };
        let album = match &resp.now.releases {
            Some(releases) => releases.get(0).map_or("Unknown Album".to_string(), |r| r.title.clone()),
            None => "Unknown Album".to_string(),
        };
        (title, artist, album)
    } else {
        ("No current song".to_string(), "N/A".to_string(), "N/A".to_string())
    };

    Ok(Song {
        title,
        artist,
        played_time: resp.now.played_time,
        album,
    })
}

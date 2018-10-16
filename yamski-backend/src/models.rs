use std::cmp::Ordering;
use std::path::PathBuf;
use std::net::IpAddr;
use std::sync::RwLock;

use std::collections::HashMap;

use chrono::{DateTime, Local};

use url::Url;

#[derive(Debug)]
pub struct MusicState {
    pub users: RwLock<HashMap<IpAddr, String>>,
    pub playlist: RwLock<Vec<PlaylistItem>>,
}

impl MusicState {
    pub fn new() -> MusicState {
        MusicState {
            users: RwLock::new(HashMap::new()),
            playlist: RwLock::new(Vec::new()),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Alias {
    pub alias: String,
}

#[derive(Serialize, Debug)]
pub struct Playlist {
    pub list: Vec<PublicPlaylistItem>,
}

#[derive(Debug, Clone)]
pub struct PlaylistItem {
    pub title: String,
    pub file: PathBuf,
    pub source_url: Option<Url>,
    pub duration: i32,
    pub user: IpAddr,
    pub submitted: DateTime<Local>,
}

#[derive(Debug, Serialize)]
pub struct PublicPlaylistItem {
    pub title: String,
    pub file: PathBuf,
    #[serde(with = "url_serde")]
    pub source_url: Option<Url>,
    pub duration: i32,
    pub user: IpAddr,
    pub client_submitted: bool,
    pub submitted: DateTime<Local>,
}

impl PublicPlaylistItem {
    pub fn from(old: PlaylistItem, remote: IpAddr) -> PublicPlaylistItem {
        PublicPlaylistItem {
            title: old.title,
            file: old.file,
            source_url: old.source_url,
            duration: old.duration,
            user: old.user,
            submitted: old.submitted,
            client_submitted: old.user == remote
        }
    }
}

impl PartialEq for PlaylistItem {
    fn eq(&self, other: &PlaylistItem) -> bool {
        self.user == other.user && self.submitted == other.submitted
    }
}

impl Eq for PlaylistItem {}

impl PartialOrd for PlaylistItem {
    fn partial_cmp(&self, other: &PlaylistItem) -> Option<Ordering> {
        self.submitted.partial_cmp(&other.submitted)
    }
}

impl Ord for PlaylistItem {
    fn cmp(&self, other: &PlaylistItem) -> Ordering {
        self.submitted.cmp(&other.submitted)
    }
}

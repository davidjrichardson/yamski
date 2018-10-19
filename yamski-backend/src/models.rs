use std::cmp::Ordering;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::{RwLock, RwLockReadGuard};

use std::collections::HashMap;

use chrono::{DateTime, Local};

use url::Url;

#[derive(Debug, Serialize, Clone)]
pub enum DownloadState {
    NotStarted,
    InProgress,
    Complete,
}

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
    pub source_url: Url,
    pub duration: i32,
    pub user: IpAddr,
    pub submitted: DateTime<Local>,
    pub downloading: DownloadState,
}

#[derive(Debug, Serialize)]
pub struct PublicPlaylistItem {
    pub title: String,
    pub duration: i32,
    pub user: String,
    pub client_submitted: bool,
    pub submitted: DateTime<Local>,
    pub downloading: DownloadState,
}

impl PublicPlaylistItem {
    pub fn from(
        old: PlaylistItem,
        remote: IpAddr,
        user_map: &RwLockReadGuard<HashMap<IpAddr, String>>,
    ) -> PublicPlaylistItem {
        PublicPlaylistItem {
            title: old.title,
            duration: old.duration,
            user: user_map.get(&remote).unwrap().to_string(),
            submitted: old.submitted,
            client_submitted: old.user == remote,
            downloading: old.downloading,
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

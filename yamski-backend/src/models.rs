use std::cmp::Ordering;
use std::fs::File;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};

use chrono::{DateTime, Local};

use url::Url;

#[derive(Debug)]
pub struct MusicState {
    pub user_list: RwLock<Vec<Arc<User>>>,
    pub playlist: RwLock<Vec<Arc<PlaylistItem>>>,
}

impl MusicState {
    pub fn new() -> MusicState {
        MusicState {
            user_list: RwLock::new(Vec::new()),
            playlist: RwLock::new(Vec::new()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct User {
    pub address: SocketAddr,
    pub alias: String,
}

#[derive(Debug)]
pub struct PlaylistItem {
    pub title: String,
    pub file: File,
    pub source_url: Option<Url>,
    pub duration: i32,
    pub user: Arc<User>,
    pub submitted: DateTime<Local>,
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

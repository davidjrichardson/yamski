use std::cmp::Ordering;
use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::{Arc, RwLock, RwLockReadGuard};

use std::collections::HashMap;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::{Data, Request};

use chrono::{DateTime, Local};

use rand::{thread_rng, Rng};

use url::Url;

#[derive(Debug)]
pub struct MusicState {
    pub users: RwLock<HashMap<IpAddr, String>>,
    pub playlist: RwLock<Vec<PlaylistItem>>,
    pub names: Arc<Vec<String>>,
}

impl MusicState {
    pub fn new(names: Arc<Vec<String>>) -> MusicState {
        MusicState {
            users: RwLock::new(HashMap::new()),
            playlist: RwLock::new(Vec::new()),
            names,
        }
    }
}

impl Fairing for MusicState {
    fn info(&self) -> Info {
        Info {
            name: "Random User Alias Assignment",
            kind: Kind::Request,
        }
    }

    fn on_request(&self, request: &mut Request, _data: &Data) {
        let remote = request.remote();
        
        match remote {
            Some(sock) => {
                let ip_addr = sock.ip();

                println!("Write guard");
                let mut user_write_guard = self.users.write().unwrap();

                match user_write_guard.get(&ip_addr) {
                    Some(_) => (),
                    None => {
                        let a = thread_rng()
                            .choose(&self.names)
                            .expect("Failed to choose random alias")
                            .clone();

                        user_write_guard.insert(ip_addr, a);
                        return
                    }
                }
            }
            None => (),
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
}

#[derive(Debug, Serialize)]
pub struct PublicPlaylistItem {
    pub title: String,
    pub duration: i32,
    pub user: String,
    pub client_submitted: bool,
    pub submitted: DateTime<Local>,
}

impl PublicPlaylistItem {
    pub fn from(
        old: PlaylistItem,
        remote: IpAddr,
        user_map: &RwLockReadGuard<HashMap<IpAddr, String>>,
    ) -> Result<PublicPlaylistItem, Status> {
        match user_map.get(&remote) {
            Some(alias) => Ok(PublicPlaylistItem {
                title: old.title,
                duration: old.duration,
                user: alias.clone(),
                submitted: old.submitted,
                client_submitted: old.user == remote,
            }),
            None => Err(Status::Unauthorized),
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

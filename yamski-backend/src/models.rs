use std::cmp::Ordering;
use std::fs::File;
use std::net::IpAddr;
use std::sync::{Arc, RwLock};

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

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
    pub address: IpAddr,
    pub alias: String,
}

// impl<'a, 'r> FromRequest<'a, 'r> for User {
//     type Error = ();

//     fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
//         match request.remote() {
//             Some(addr)  => {
//                 let server_state = request.guard::<State<MusicState>>()?;
//                 let user_iter = server_state.user_list.read().unwrap().into_iter();

//                 match user_iter.find(|item| item.address == addr) {
//                     Some(user)  => Outcome::Success(Arc::clone(&user)),
//                     None        => Outcome::Failure((Status::Unauthorized, ()))
//                 }
//             },
//             None        => Outcome::Failure((Status::Unauthorized, ()))
//         }
//     }
// }

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

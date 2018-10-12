// use std::ops::Deref;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

use crate::models::{User, PlaylistItem};

use std::sync::Arc;

pub struct MusicState {
    pub user_list: Vec<Arc<User>>,
    pub playlist: Vec<Arc<PlaylistItem>>,
}

pub struct ServerState(pub MusicState);

// TODO: Implement this
// impl<'a, 'r> FromRequest<'a, 'r> for ServerState {
//     type Error = ();

//     fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
//         let state = request.guard::<State<MusicState>>()?;
//         match state.get() {
//             Ok(music_state) => Outcome::Success(ServerState(music_state)),
//             Err(_)          => Outcome::Failure((Status::ServiceUnavailable, ()))
//         }
//     }
// }
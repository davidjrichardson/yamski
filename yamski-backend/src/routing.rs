extern crate rocket;

use std::net::SocketAddr;

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::State;
use rocket_contrib::Json;

use crate::forms::AliasForm;
use crate::models::{Alias, MusicState, PlaylistItem};

#[get("/")]
fn index(state: State<MusicState>) -> String {
    format!("{:?}", state)
}

#[get("/alias", format = "application/json")]
fn get_alias(state: State<MusicState>, remote: SocketAddr) -> Option<Json<Alias>> {
    let rw_guard = state.users.read().unwrap();

    match rw_guard.get(&remote.ip()) {
        Some(a) => Some(Json(Alias {
            alias: a.to_string(),
        })),
        None => None,
    }
}

#[post("/alias", data = "<alias>", format = "application/json")]
fn update_alias(
    alias: Json<AliasForm>,
    state: State<MusicState>,
    remote: SocketAddr,
) -> Custom<String> {
    let mut rw_guard = state.users.write().unwrap();

    rw_guard
        .entry(remote.ip())
        .and_modify(|a| a.clone_from(&alias.alias))
        .or_insert(alias.alias.clone());

    Custom(Status::Ok, format!("Alias set to {}", alias.alias))
}

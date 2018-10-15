extern crate rocket;

use std::net::SocketAddr;


use rocket::State;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket_contrib::Json;

use crate::forms::AliasForm;
use crate::models::{MusicState, PlaylistItem, User};

#[get("/")]
fn index(state: State<MusicState>) -> String {
    format!("{:?}", state)
}

#[post("/alias", data = "<alias>", format = "application/json")]
fn update_alias(alias: Json<AliasForm>, state: State<MusicState>, remote: SocketAddr) -> Custom<String> {
    let mut rw_guard = state.users.write().unwrap();

    rw_guard
        .entry(remote.ip())
        .and_modify(|a| a.clone_from(&alias.alias))
        .or_insert(alias.alias.clone());

    Custom(Status::Ok, format!("Alias set to {}", alias.alias))
}

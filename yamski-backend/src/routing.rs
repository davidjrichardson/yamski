extern crate rocket;

use std::net::SocketAddr;

use rocket::State;
use rocket_contrib::Json;

use crate::forms::AliasForm;
use crate::models::{MusicState, PlaylistItem, User};

#[get("/")]
fn index(state: State<MusicState>) -> String {
    format!("{:?}", state)
}

#[post("/alias", data = "<alias>", format = "application/json")]
fn update_alias(alias: Json<AliasForm>, state: State<MusicState>, remote: SocketAddr) -> String {
    // TODO: Update the aliase (or insert one if it doesn't exist)
    format!(
        "New alias: {} for addr: {:?}\n",
        alias.into_inner().alias,
        remote.ip()
    )
}

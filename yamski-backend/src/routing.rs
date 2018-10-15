extern crate rocket;

use std::net::SocketAddr;

use std::sync::Arc;

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
    let user = state
        .user_list
        .read()
        .unwrap()
        .into_iter()
        .find(|item: &Arc<User>| {
            item.address == remote.ip()
        });

    match user {
        Some(u)  => {},
        None        => {},
    }

    format!("Changing alias for {:?} to {}\n", remote.ip(), alias.alias)
}

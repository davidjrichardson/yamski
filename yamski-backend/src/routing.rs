extern crate rocket;

use rocket::State;

use crate::models::{MusicState, PlaylistItem, User};

#[get("/")]
fn index(state: State<MusicState>) -> &'static str {
    "Hello world!"
}

#[get("/hello")]
fn hello() -> &'static str {
    "Another hello!"
}
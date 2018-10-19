#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate rocket;
extern crate url;

pub mod models;

mod forms;
mod routing;

use std::sync::{mpsc, Arc};
use std::thread;

use crate::models::PlaylistItem;

struct PlaylistChannel(mpsc::SyncSender<PlaylistItem>);

fn process_playlist_item(trigger: mpsc::Receiver<PlaylistItem>) {
    print!("Worker thread invoked: {:?}", trigger.recv().unwrap());
}

fn main() {
    let (tx, rx) = mpsc::sync_channel(1);

    thread::spawn(move || process_playlist_item(rx));

    rocket::ignite()
        .mount(
            "/",
            routes![
                routing::index,
                routing::update_alias,
                routing::get_alias,
                routing::get_playlist,
                routing::add_playlist_item,
            ],
        )
        // .manage(queue)
        .manage(models::MusicState::new())
        .manage(PlaylistChannel(tx))
        .launch();
}

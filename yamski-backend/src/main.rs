#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

extern crate chrono;
extern crate csv;
extern crate rand;
extern crate rocket;
extern crate url;

pub mod models;

mod forms;
mod routing;

use std::sync::{mpsc, Arc};
use std::thread;

use csv::Reader;

use crate::models::PlaylistItem;

struct PlaylistChannel(mpsc::SyncSender<PlaylistItem>);

fn process_playlist_item(trigger: mpsc::Receiver<PlaylistItem>) {
    // TODO: Download the video (output status ETC to console)
    print!("Worker thread invoked: {:?}", trigger.recv().unwrap());
}

fn main() {
    let mut names_csv = Reader::from_path("names.csv").expect("Couldn't open names.csv");
    let names = names_csv
        .records()
        .filter_map(|item: Result<_, _>| item.ok())
        .map(|item| item.get(0).unwrap().to_string())
        .collect::<Vec<_>>();
    let names_arc = Arc::new(names);

    let state = Arc::new(models::MusicState::new(names_arc));

    // Worker thread init
    let (tx, rx) = mpsc::sync_channel(1);
    thread::spawn(move || process_playlist_item(rx));

    // Web server init
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
        .attach(state.clone())
        .manage(state)
        .manage(PlaylistChannel(tx))
        .launch();
}

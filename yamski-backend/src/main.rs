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

use std::fs::remove_dir_all;
use std::process::{Command, Stdio};
use std::path::Path;

use crate::models::PlaylistItem;

struct PlaylistChannel(mpsc::SyncSender<PlaylistItem>);

fn process_playlist_item(trigger: mpsc::Receiver<PlaylistItem>) {
    let item = trigger.recv().expect("Was triggered but received nothing");
    println!("Downloading {}", item.title);

    let file_path = item.file.to_str().unwrap();
    let mut download_cmd = Command::new("youtube-dl")
        .arg("--no-playlist")
        .arg("-o")
        .arg(file_path)
        .arg(item.source_url.to_string())
        .stdout(Stdio::null())
        .spawn()
        .expect(
            format!(
                "Failed to download video at {}",
                item.source_url.to_string()
            )
            .as_str(),
        );

    println!(
        "{}",
        download_cmd
            .wait()
            .and_then(|_x| Ok(format!("{} downloaded successfully", item.title)))
            .expect("Failed to execute download")
    );
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

    if Path::new("tmp/").exists() {
        println!("Cleaning previous videos");
        remove_dir_all("tmp/").expect("Failed to clean video directory");
    }

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

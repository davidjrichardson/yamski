extern crate rocket;

use std::net::SocketAddr;
use std::process::Command;

use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::State;
use rocket_contrib::Json;

use std::path::PathBuf;

use chrono::{Duration, Local};

use url::Url;

use crate::forms::{AliasForm, PlaylistItemForm};
use crate::models::{Alias, DownloadState, MusicState, Playlist, PlaylistItem, PublicPlaylistItem};
use crate::PlaylistChannel;

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

fn duration_str_to_triple(duration: String) -> (i64, i64, i64) {
    let mut h = 0;
    let mut m = 0;
    let mut s = 0;

    match duration.matches(":").count() {
        2 => {
            let components = duration.split(":").collect::<Vec<_>>();
            h = components.first().unwrap().parse().unwrap();
            m = components.first().unwrap().parse().unwrap();
            s = components.first().unwrap().parse().unwrap();
        }
        1 => {
            let components = duration.split(":").collect::<Vec<_>>();
            m = components.first().unwrap().parse().unwrap();
            s = components.first().unwrap().parse().unwrap();
        }
        0 => {
            s = duration.parse().unwrap();
        }
        _ => {}
    }

    (h, m, s)
}

fn get_duration(url: &Url) -> Duration {
    let command = Command::new("youtube-dl")
        .args(&["--get-duration", "--no-playlist", "--skip-download"])
        .arg(url.to_string())
        .output()
        .expect(format!("youtube-dl failed to get duration for {}", url.to_string()).as_str());
    let duration_str = String::from_utf8_lossy(&command.stdout);
    let duration = duration_str.trim_right().trim_left();

    let (h, m, s) = duration_str_to_triple(duration.to_string());

    Duration::hours(h) + Duration::minutes(m) + Duration::seconds(s)
}

fn get_title(url: &Url) -> String {
    let command = Command::new("youtube-dl")
        .args(&["--get-title", "--no-playlist", "--skip-download"])
        .arg(url.to_string())
        .output()
        .expect(format!("youtube-dl failed to get duration for {}", url.to_string()).as_str());
    let title_str = String::from_utf8_lossy(&command.stdout);
    title_str.trim_right().trim_left().to_string()
}

#[post("/playlist", data = "<item>", format = "application/json")]
fn add_playlist_item(
    item: Json<PlaylistItemForm>,
    state: State<MusicState>,
    msg_queue: State<PlaylistChannel>,
    remote: SocketAddr,
) -> Result<Json<PublicPlaylistItem>, Status> {
    match Url::parse(item.source_url.as_str()) {
        Ok(source_url) => {
            let duration = get_duration(&source_url);
            let title = get_title(&source_url);

            let playlist_item = PlaylistItem {
                title: title,
                duration: duration.num_seconds() as i32,
                source_url: source_url.clone(),
                user: remote.ip(),
                submitted: Local::now(),
                downloading: DownloadState::NotStarted,
                file: PathBuf::new(),
            };

            println!("user alias guard unwrap");
            let user_alias_guard = state.users.read().unwrap();
            let public_playlist_item =
                PublicPlaylistItem::from(playlist_item.clone(), remote.ip(), &user_alias_guard);

            match msg_queue.0.try_send(playlist_item.clone()) {
                Ok(_) => {
                    println!("playlist guard unwrap");
                    let mut playist_guard = state.playlist.write().unwrap();
                    playist_guard.push(playlist_item);

                    Ok(Json(public_playlist_item))
                }
                Err(_) => Err(Status::BadRequest),
            }
        }
        Err(_) => Err(Status::BadRequest),
    }
}

#[get("/playlist", format = "application/json")]
fn get_playlist(state: State<MusicState>, remote: SocketAddr) -> Json<Playlist> {
    let rw_guard = state.playlist.read().unwrap();
    let users_rw_guard = state.users.read().unwrap();

    Json(Playlist {
        list: rw_guard
            .to_vec()
            .iter()
            .map(|item| PublicPlaylistItem::from(item.clone(), remote.ip(), &users_rw_guard))
            .collect(),
    })
}

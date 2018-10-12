#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate chrono;
extern crate url;

pub mod models;

mod routing;

fn main() {
    rocket::ignite()
        .mount("/", routes![routing::index, routing::hello])
        .manage(models::MusicState::new())
        .launch();
}

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

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                routing::index,
                routing::update_alias,
                routing::get_alias,
                routing::get_playlist,
            ],
        )
        .manage(models::MusicState::new())
        .launch();
}

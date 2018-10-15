#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

extern crate rocket;
extern crate chrono;
extern crate url;

pub mod models;

mod forms;
mod routing;

fn main() {
    rocket::ignite()
        .mount("/", routes![routing::index, routing::update_alias])
        .manage(models::MusicState::new())
        .launch();
}

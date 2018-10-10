#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate diesel;

pub mod guards;
mod routing;

fn main() {
    rocket::ignite()
        .manage(guards::init_pool())
        .mount("/", routes![routing::index, routing::hello])
        .mount("/test", routes![routing::hello])
        .launch();
}

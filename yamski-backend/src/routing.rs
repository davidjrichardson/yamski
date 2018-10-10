extern crate rocket;

use crate::guards::DbConn;

#[get("/")]
fn index(conn: DbConn) -> &'static str {
    "Hello, world!"
}

#[get("/hello")]
fn hello() -> &'static str {
    "Another hello!"
}
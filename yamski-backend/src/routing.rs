extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello")]
fn hello() -> &'static str {
    "Another hello!"
}
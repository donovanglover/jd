#![doc = include_str!("../README.md")]

#[macro_use]
extern crate rocket;

/// Start a new web server
#[rocket::main]
pub async fn init() {
    let _ = rocket::build().mount("/", routes![index]).launch().await;
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    println!("Integrate with CLI or keep separate?")
}

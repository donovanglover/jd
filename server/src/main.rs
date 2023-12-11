#[rocket::main]
pub async fn init() {
    let result = rocket::build()
        .mount("/", routes![index])
        .launch()
        .await;
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    println!("Integrate with CLI or keep separate?")
}

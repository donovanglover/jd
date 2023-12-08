#[rocket::main]
pub async fn init() {
    let result = rocket::build().launch().await;
}

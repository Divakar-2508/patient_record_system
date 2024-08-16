mod api;
mod error;
mod hasher;
mod jwt;
mod models;
mod record_client;
mod response;

use api::*;
use rocket::routes;

#[rocket::launch]
async fn launch() -> _ {
    dotenv::dotenv().ok();

    let database = record_client::RecordClient::new()
        .await
        .expect("Can't init database");

    rocket::build()
        .mount("/", routes![hello_world, login_user])
        .manage(database)
}

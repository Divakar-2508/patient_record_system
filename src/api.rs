use rocket::{get, post, serde::json::Json, State};

use crate::{
    hasher,
    models::{Doctor, LoginPayload},
    record_client::RecordClient,
    response::RecordResponse,
};

#[get("/hello")]
pub fn hello_world() -> String {
    "Hello".into()
}

#[post("/register_doctor")]
pub async fn register_doctor(
    payload: Json<Doctor>,
    client: &State<RecordClient>,
) -> RecordResponse {
    if let Err(err) = client.register_doctor(&payload.into_inner()).await {
        return RecordResponse::LoginFailed(err);
    }
}

#[post("/login_user", data = "<payload>")]
pub fn login_user(payload: Json<LoginPayload>, client: &State<RecordClient>) -> RecordResponse {
    todo!("Validate login");
}

use rocket::{serde::json::Json, Responder};

use crate::{models::PatientRecord, record_client::Doctor};

#[derive(Responder)]
pub enum Error {
    #[response(status = 404)]
    UserNotFound(String),
    #[response(status = 429)]
    RegistrationFailed(String),
}

#[derive(Responder)]
pub enum Success {
    #[response(status = 200)]
    RegisterSuccess(String),
    #[response(status = 200, content_type = "json")]
    PatientDetails(Json<PatientRecord>),
    #[response(status = 200, content_type = "json")]
    DoctorDetails(Json<Doctor>),
}

#[derive(Responder)]
pub enum Response {
    Success(Success),
    Error(Error),
}

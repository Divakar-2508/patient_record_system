pub mod models;
pub mod record_client;
pub mod response;

use record_client::PatientRecordManager;
use response::{Error, Response, Success};
use rocket::{form::Form, get, launch, post, routes, serde::json::Json, FromForm, State};

#[derive(FromForm)]
pub struct PatientDetails {
    patient_name: String,
    patient_age: usize,
}

#[derive(FromForm)]
pub struct DoctorDetails {
    doctor_name: String,
    hospital_name: String,
    hospital_address: String,
}

#[post("/register/patient", data = "<patient_data>")]
async fn register_patient(
    client: &State<PatientRecordManager>,
    patient_data: Form<PatientDetails>,
) -> Response {
    let result = client.register_patient(patient_data.into_inner()).await;

    match result {
        Ok(patient_id) => Response::Success(Success::RegisterSuccess(format!("{}", patient_id))),
        Err(err) => Response::Error(Error::RegistrationFailed(format!(
            "Failed: {}",
            err.to_string()
        ))),
    }
}

#[post("/register/doctor", data = "<doctor_data>")]
async fn register_doctor(
    client: &State<PatientRecordManager>,
    doctor_data: Form<DoctorDetails>,
) -> Response {
    let result = client.register_doctor(doctor_data.into_inner()).await;

    match result {
        Ok(doctor_id) => Response::Success(Success::RegisterSuccess(format!("{}", doctor_id))),
        Err(err) => Response::Error(Error::RegistrationFailed(format!(
            "Register Failed: {}",
            err.to_string()
        ))),
    }
}

#[get("/get_diagnosis/<patient_id>")]
async fn get_diagnosis_reports(
    patient_id: usize,
    client: &State<PatientRecordManager>,
) -> Response {
    let diagnosis_reports_result = client.get_diagnosis_reports(patient_id).await;
    match diagnosis_reports_result {
        Ok(report) => Response::Success(Success::PatientDetails(Json(report))),
        Err(err) => Response::Error(Error::UserNotFound(err.to_string())),
    }
}

#[get("/get_doctor_details/<doctor_id>")]
async fn get_doctor_details(doctor_id: usize, client: &State<PatientRecordManager>) -> Response {
    let doctor_details_result = client.get_doctor_by_id(doctor_id).await;

    match doctor_details_result {
        Ok(doctor_detail) => Response::Success(Success::DoctorDetails(Json(doctor_detail))),
        Err(err) => Response::Error(Error::UserNotFound(err.to_string())),
    }
}

#[launch]
#[tokio::main]
async fn rocket() -> _ {
    dotenv::dotenv().ok();
    let connection_url = dotenv::var("connection_url").unwrap();
    let record_client = PatientRecordManager::new_with_uri(&connection_url)
        .await
        .unwrap();

    rocket::build()
        .mount(
            "/",
            routes![
                register_patient,
                register_doctor,
                get_diagnosis_reports,
                get_doctor_details
            ],
        )
        .manage(record_client)
}

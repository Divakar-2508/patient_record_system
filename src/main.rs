pub mod record_client;

use record_client::PatientRecordManager;
use rocket::{form::Form, get, launch, post, routes, FromForm, Responder, State};

#[get("/")]
async fn index() -> &'static str {
    "Hello World"
}

#[derive(Responder)]
enum Response {
    #[response(status = 200)]
    RegisterSuccess(String),

    RegisterFailed(String),
    #[response(status = 404)]
    RequestFailed(String),
    #[response(status = 200)]
    PatientDetails(())
}

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
    let result = client
        .register_patient(patient_data.into_inner())
        .await;

    match result {
        Ok(patient_id) => Response::RegisterSuccess(patient_id.to_string()),
        Err(err) => Response::RegisterFailed(format!("Failed: {}", err.to_string())),
    }
}

#[post("/register/doctor", data="<doctor_data>")]
async fn register_doctor(
    client: &State<PatientRecordManager>,
    doctor_data: Form<DoctorDetails>,
) -> Response {
    let result = client
        .register_doctor(doctor_data.into_inner())
        .await;

    match result {
        Ok(doctor_id) => {
            Response::RegisterSuccess(doctor_id.to_string())
        },
        Err(err) => {
            Response::RegisterFailed(format!("Register Failed: {}", err.to_string()))
        }
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
        .mount("/", routes![index, register_patient, register_doctor])
        .manage(record_client)
}
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct Doctor {
    pub name: String,
    pub email: String,
    pub password: String,
    pub hospital_name: String,
    pub hospital_address: String,
}

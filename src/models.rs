use mongodb::bson::{bson, Bson};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PatientRecord {
    patient_id: usize,
    patient_name: String,
    age: usize,
    diagnosis_reports: Vec<DiagnosisReport>,
}

impl PatientRecord {
    pub(crate) fn new(
        patient_id: usize,
        patient_name: String,
        age: usize,
        diagnosis_reports: Vec<DiagnosisReport>,
    ) -> Self {
        Self {
            patient_id,
            patient_name: patient_name.to_string(),
            age,
            diagnosis_reports,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosisReport {
    doctor_id: usize,
    doctor_name: String,
    details: String,
}

impl DiagnosisReport {
    pub fn new(doctor_id: usize, doctor_name: String, details: String) -> Self {
        Self {
            doctor_id,
            doctor_name,
            details,
        }
    }
}

impl Into<Bson> for DiagnosisReport {
    fn into(self) -> Bson {
        bson!({
            "doctor_id": self.doctor_id as i32,
            "doctor_name": self.doctor_name,
            "details": self.details
        })
    }
}

use mongodb::{
    bson::{doc, Document},
    options::{FindOneAndUpdateOptions, IndexOptions},
    Client, Collection, Database, IndexModel,
};
use rocket::futures::StreamExt;
use serde::{Deserialize, Serialize};

use crate::{
    models::{DiagnosisReport, PatientRecord},
    DoctorDetails, PatientDetails,
};

pub struct PatientRecordManager {
    database: Database,
}

impl PatientRecordManager {
    pub async fn new_with_uri(uri: &str) -> mongodb::error::Result<Self> {
        let client = Client::with_uri_str(uri).await?;
        let database = client.database("patient_records");
        let patient_collection: Collection<PatientRecord> = database.collection("record");

        let index_option = IndexOptions::builder().unique(true).build();
        let patient_index_model = IndexModel::builder()
            .keys(doc! {
                "patient_id": 1,
            })
            .options(index_option.clone())
            .build();

        patient_collection
            .create_index(patient_index_model, None)
            .await?;

        let doctor_collection: Collection<Doctor> = database.collection("doctor");
        let doctor_index_model = IndexModel::builder()
            .keys(doc! {
                "doctor_id": 1,
            })
            .options(index_option)
            .build();
        doctor_collection
            .create_index(doctor_index_model, None)
            .await?;

        Ok(Self { database })
    }

    pub async fn register_patient(&self, patient_details: PatientDetails) -> Result<usize, String> {
        let collection: Collection<PatientRecord> = self.database.collection("record");
        let counter_collection: Collection<Document> = self.database.collection("patient_counter");

        let update_doc = doc! {
            "$inc": {
                "count": 1
            }
        };
        let find_and_modify_options = FindOneAndUpdateOptions::builder()
            .upsert(true)
            .return_document(mongodb::options::ReturnDocument::After)
            .build();

        let patient_id = counter_collection
            .find_one_and_update(doc! {}, update_doc, find_and_modify_options)
            .await;
        if let Err(err) = patient_id {
            return Err(format!("{}", err.to_string()));
        }

        let patient_id = match patient_id.unwrap() {
            Some(result) => {
                let id = result.get("count").unwrap().as_i32().unwrap();
                id
            }
            None => {
                return Err("No doctor ID found".to_string());
            }
        };

        let patient_record = PatientRecord::new(
            patient_id as usize,
            patient_details.patient_name.to_lowercase(),
            patient_details.patient_age,
            Vec::new(),
        );

        match collection.insert_one(patient_record, None).await {
            Ok(_) => Ok(patient_id as usize),
            Err(err) => {
                return Err(format!("Error while inserting, {}", err.to_string()));
            }
        }
    }

    pub async fn add_diagnosis_reports(
        &self,
        patient_id: usize,
        report: DiagnosisReport,
    ) -> Result<(), String> {
        let collection: Collection<PatientRecord> = self.database.collection("record");

        let filter = doc! {
            "patient_id" : patient_id as i32,
        };

        let update = doc! {
            "$push": {
                "diagnosis_reports": report
            }
        };

        let update_result = collection.update_one(filter, update, None).await;

        match update_result {
            Ok(result) => {
                if result.matched_count == 1 {
                    Ok(())
                } else {
                    Err("No Record with given id is found".to_string())
                }
            }
            Err(err) => Err(format!("Ran into Error: {}", err.to_string())),
        }
    }

    pub async fn get_diagnosis_reports(&self, patient_id: usize) -> Result<PatientRecord, String> {
        let collection: Collection<PatientRecord> = self.database.collection("record");

        let filter = doc! {
            "patient_id": patient_id as i32,
        };

        let mut cursor = collection
            .find(filter, None)
            .await
            .map_err(|err| err.to_string())?;

        if let Some(result) = cursor.next().await {
            result.map_err(|err| err.to_string())
        } else {
            Err("Report Not Found".into())
        }
    }

    pub async fn get_patient_by_id(&self, patient_id: usize) -> Result<PatientRecord, String> {
        let collection: Collection<PatientRecord> = self.database.collection("record");
        let filter = doc! {
            "patient_id": patient_id as i32,
        };

        let record_result = collection.find_one(filter, None).await;

        match record_result {
            Ok(record) => {
                if record.is_some() {
                    Ok(record.unwrap())
                } else {
                    Err("No Record with given id is found".to_string())
                }
            }
            Err(err) => Err(format!("Ran into Error: {}", err.to_string())),
        }
    }

    pub async fn get_patients_by_name(
        &self,
        patient_name: &str,
    ) -> Result<Vec<PatientRecord>, String> {
        let collection: Collection<PatientRecord> = self.database.collection("record");

        let filter = doc! {
            "patient_name": patient_name,
        };

        let record_result = collection.find(filter, None).await;

        match record_result {
            Ok(mut cursor) => {
                let mut records = Vec::new();
                while let Ok(true) = cursor.advance().await {
                    records.push(cursor.deserialize_current().unwrap());
                }
                Ok(records)
            }
            Err(err) => Err(format!("Ran into Error: {}", err.to_string())),
        }
    }

    pub async fn register_doctor(&self, doctor_details: DoctorDetails) -> Result<usize, String> {
        let collection: Collection<Doctor> = self.database.collection("doctor");

        let counter_collection: Collection<Document> = self.database.collection("doctor_count");

        let update_doc = doc! {
            "$inc": {
                "count": 1
            }
        };
        let find_and_modify_options = FindOneAndUpdateOptions::builder()
            .upsert(true)
            .return_document(mongodb::options::ReturnDocument::After)
            .build();

        let doctor_id = counter_collection
            .find_one_and_update(doc! {}, update_doc, find_and_modify_options)
            .await;
        if let Err(err) = doctor_id {
            return Err(format!("{}", err.to_string()));
        }

        let doctor_id = match doctor_id.unwrap() {
            Some(result) => {
                let id = result.get("count").unwrap().as_i32().unwrap();
                id
            }
            None => {
                return Err("No doctor ID found".to_string());
            }
        };

        let doctor = Doctor::new(
            doctor_id as usize,
            doctor_details.doctor_name.to_lowercase(),
            doctor_details.hospital_name,
            doctor_details.hospital_address,
        );

        match collection.insert_one(doctor, None).await {
            Ok(_) => Ok(doctor_id as usize),
            Err(err) => {
                return Err(format!("Error while inserting, {}", err.to_string()));
            }
        }
    }

    pub async fn get_doctor_by_id(&self, doctor_id: usize) -> Result<Doctor, String> {
        let collection: Collection<Doctor> = self.database.collection("doctor");

        let filter = doc! {
            "doctor_id": doctor_id as i32,
        };

        match collection.find_one(filter, None).await {
            Ok(record) => record.ok_or("No Entry Found".to_string()),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Doctor {
    doctor_id: usize,
    doctor_name: String,
    hospital_name: String,
    hospital_address: String,
}

impl Doctor {
    pub fn new(
        doctor_id: usize,
        doctor_name: String,
        hospital_name: String,
        hospital_address: String,
    ) -> Self {
        Self {
            doctor_id,
            doctor_name,
            hospital_name,
            hospital_address,
        }
    }
}

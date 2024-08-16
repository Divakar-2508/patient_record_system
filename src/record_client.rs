use mongodb::{bson::doc, Client, Collection, Database};

use crate::hasher::RecordHasher;
use crate::{
    error::RecordError,
    models::{Doctor, LoginPayload},
};

pub struct RecordClient {
    database: Database,
    hasher: RecordHasher,
}

impl RecordClient {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        let client_uri = std::env::var("MONGO_DB_URI")
            .expect("Can't find MONGO_DB_URI, set in environmental variables");

        let client = Client::with_uri_str(client_uri).await?;

        let database_name = std::env::var("DATABASE_NAME")
            .expect("can't find DATABASE_NAME set in environmental variables");

        let database = client.database(&database_name);

        database
            .collection("hello")
            .insert_one(doc! {
                "some": "hello"
            })
            .await?;

        let hasher = RecordHasher::default();
        Ok(Self { database, hasher })
    }

    pub async fn register_doctor(&self, doctor: &Doctor) -> Result<(), RecordError> {
        let conn: Collection<Doctor> = self.database.collection("doctor_details");

        conn.insert_one(doctor)
            .await
            .map(|_| ())
            .map_err(|err| RecordError::DatabaseError(err.to_string()))
    }

    pub async fn validate_doctor_details(
        &self,
        login_details: LoginPayload,
    ) -> Result<String, RecordError> {
        let collection: Collection<Doctor> = self.database.collection("doctor_details");

        let filter = doc! {
            "email": login_details.email,
        };

        let doctor = collection
            .find_one(filter)
            .await
            .map_err(|err| RecordError::DatabaseError(err.to_string()))?;

        match doctor {
            Some(doctor_data) => {
                if let Ok(_) = self
                    .hasher
                    .verify_password(&login_details.password, &doctor_data.password)
                {
                    Ok(doctor_data.name)
                } else {
                    Err(RecordError::BadCredentials)
                }
            }
            None => Err(RecordError::UserNotFound),
        }
    }
}

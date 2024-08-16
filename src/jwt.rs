use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: usize,
    email: String,
    exp: usize,
}

pub fn generate_jwt(id: usize, email: String) -> Result<String, jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").expect("secret not found");

    let current_time = Utc::now()
        .checked_add_signed(Duration::hours(3))
        .expect("Invalid Time Stamp")
        .timestamp();

    let claims = Claims {
        id,
        email,
        exp: current_time as usize,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

pub fn check_jwt(token: String) -> Result<(), jsonwebtoken::errors::Error> {
    let secret = std::env::var("JWT_SECRET").expect("secret not found");

    jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|_| ())
}

#[test]
fn test_generate_jwt() {
    dotenv::dotenv().ok();

    let jwt_token = generate_jwt(25, "hello@gmail.com".to_string());
    assert!(jwt_token.is_ok(), "Can't generate jwt");
}

#[test]
fn test_validate_jwt() {
    dotenv::dotenv().ok();

    let jwt_token = generate_jwt(25, "Some@gmail.com".to_string()).unwrap();

    assert!(check_jwt(jwt_token).is_ok(), "Not Ok");
}

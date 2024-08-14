use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub struct RecordHasher<'a> {
    argon: Argon2<'a>,
}

impl<'a> RecordHasher<'a> {
    pub fn new() -> Self {
        Self {
            argon: Argon2::default(),
        }
    }

    pub fn hash_password(&self, password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);

        self.argon
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
    }

    pub fn verify_password(
        &self,
        password: &str,
        hash: &str,
    ) -> Result<(), argon2::password_hash::Error> {
        let hashed_password = PasswordHash::new(&hash)?;

        self.argon
            .verify_password(password.as_bytes(), &hashed_password)
    }
}

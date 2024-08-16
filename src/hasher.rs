use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};

pub struct RecordHasher {
    argon: Argon2<'static>,
}

impl Default for RecordHasher {
    fn default() -> Self {
        Self {
            argon: Argon2::default(),
        }
    }
}

impl RecordHasher {
    pub fn hash_password(&self, password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);

        self.argon
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
    }

    pub fn verify_password(
        &self,
        password: &str,
        original_hash: &str,
    ) -> Result<(), argon2::password_hash::Error> {
        let password_hash = PasswordHash::new(original_hash)?;

        self.argon
            .verify_password(password.as_bytes(), &password_hash)
    }
}

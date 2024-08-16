pub enum RecordError {
    UserNotFound,
    BadCredentials,
    DatabaseError(String),
}

use rocket::Responder;

#[derive(Responder)]
pub enum RecordResponse {
    #[response(status = 200)]
    LoginSuccess(String),
    #[response(status = 401)]
    LoginFailed(String),
}

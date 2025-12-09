use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse<T: Serialize> {
    message: String,
    detail: Option<T>
}

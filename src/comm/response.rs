use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SuccessResp<T> {
    pub status: i16,
    pub message: String,
    pub data: T,
}

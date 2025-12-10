use serde::Serialize;
use serde::Deserialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequestData {
    pub email: String,
    pub password: String,
}
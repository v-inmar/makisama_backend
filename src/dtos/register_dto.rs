use serde::Deserialize;
use serde::Serialize;

use validator::Validate;

use crate::utils::custom_validation_utils::validate_email;
use crate::utils::custom_validation_utils::validate_name;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct RegisterRequestData {
    #[validate(custom(function = "validate_name"))]
    pub firstname: String,

    #[validate(custom(function = "validate_name"))]
    pub lastname: String,

    #[validate(custom(function = "validate_email"))]
    pub email: String,

    #[validate(length(min = 8, max = 255))]
    pub password: String,

    #[validate(length(min = 8, max = 255))]
    pub repeat: String,
}

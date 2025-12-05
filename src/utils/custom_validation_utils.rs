use std::str::FromStr;

use email_address::EmailAddress;
use validator::ValidationError;

pub fn validate_name(name: &str) -> Result<(), ValidationError> {
    if name.trim().len() < 3 || name.trim().len() > 255 {
        return Err(ValidationError::new("Must be between 3 and 255 characters"));
    }

    if name.chars().any(|c| c.is_digit(10)) {
        return Err(ValidationError::new("Must not contain numbers"));
    }

    Ok(())
}

pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    if EmailAddress::from_str(email).is_err() {
        return Err(ValidationError::new("Invalid format"));
    }

    if email.trim().len() > 255 {
        return Err(ValidationError::new("Must not exceed 255 characters"));
    }

    Ok(())
}

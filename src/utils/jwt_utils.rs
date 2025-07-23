use std::env;

use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode,
};
use serde::{Deserialize, Serialize};

pub enum TokenType {
    Access,
    Refresh,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (the auth token)
    pub exp: i64,           // Expiration time
    pub iat: i64,           // Issued at time
    pub token_type: String, // access or refresh
}

fn _generate_jwt(
    auth_identity_value: &str,
    token_type_enum: &TokenType,
    exp_time: DateTime<Utc>,
    secret: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let utc_now = Utc::now();

    let token_type = match token_type_enum {
        TokenType::Access => "access",
        TokenType::Refresh => "refresh",
    };

    // Set the claims
    let claims = Claims {
        sub: auth_identity_value.to_string(),
        exp: exp_time.timestamp(),
        iat: utc_now.timestamp(),
        token_type: token_type.to_string(),
    };

    // encode to get jwt token
    let token = encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;

    Ok(token)
}

fn _decode_jwt(token: &str, secret: &str) -> Result<TokenData<Claims>, Box<dyn std::error::Error>> {
    let mut validation = Validation::new(Algorithm::HS512);
    validation.validate_exp = true;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;

    Ok(token_data)
}

fn _decode_jwt_no_exp_validation(
    token: &str,
    secret: &str,
) -> Result<TokenData<Claims>, Box<dyn std::error::Error>> {
    let mut validation = Validation::new(Algorithm::HS512);
    validation.validate_exp = false;

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    )?;

    Ok(token_data)
}

pub fn generate_access_token(
    auth_identity_value: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // get secret
    let access_secret = env::var("JWT_ACCESS_SECRET").map_err(|e| {
        log::error!("{}", e);
        Box::new(e)
    })?;

    // get exp time
    let exp_minutes = env::var("ACCESS_TOKEN_EXPIRATION_MINUTES")
        .map_err(|e| {
            log::error!("{}", e);
            Box::new(e)
        })?
        .parse::<i64>()
        .map_err(|e| {
            log::error!(
                "Failed to parse ACCESS_TOKEN_EXPIRATION_MINUTES to i64: {}",
                e
            );
            Box::new(e)
        })?;

    // set the expiration time to be used into the token
    let utc_now = Utc::now();
    let expiration = utc_now + Duration::minutes(exp_minutes);

    // get access token
    let token = _generate_jwt(
        auth_identity_value,
        &TokenType::Access,
        expiration,
        &access_secret,
    )?;

    Ok(token)
}

pub fn generate_refresh_token(
    auth_identity_value: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    // get secret
    let refresh_secret = env::var("JWT_REFRESH_SECRET").map_err(|e| {
        log::error!("{}", e);
        Box::new(e)
    })?;

    // get exp time
    let exp_days = env::var("REFRESH_TOKEN_EXPIRATION_DAYS")
        .map_err(|e| {
            log::error!("{}", e);
            Box::new(e)
        })?
        .parse::<i64>()
        .map_err(|e| {
            log::error!(
                "Failed to parse REFRESH_TOKEN_EXPIRATION_DAYS to i64: {}",
                e
            );
            Box::new(e)
        })?;

    // set the expiration time to be used into the token
    let utc_now = Utc::now();
    let expiration = utc_now + Duration::days(exp_days);

    // get access token
    let token = _generate_jwt(
        auth_identity_value,
        &TokenType::Refresh,
        expiration,
        &refresh_secret,
    )?;

    Ok(token)
}

pub fn decode_access_token(token: &str) -> Result<TokenData<Claims>, Box<dyn std::error::Error>> {
    let secret = env::var("JWT_ACCESS_SECRET").map_err(|e| {
        log::error!("{}", e);
        Box::new(e)
    })?;

    let token_data = _decode_jwt(token, &secret)?;

    Ok(token_data)
}

// only useful for getting the claims even when token had expired
pub fn decode_access_token_no_validation_exp(
    token: &str,
) -> Result<TokenData<Claims>, Box<dyn std::error::Error>> {
    let secret = env::var("JWT_ACCESS_SECRET").map_err(|e| {
        log::error!("{}", e);
        Box::new(e)
    })?;

    let token_data = _decode_jwt_no_exp_validation(token, &secret)?;

    Ok(token_data)
}

pub fn decode_refresh_token(token: &str) -> Result<TokenData<Claims>, Box<dyn std::error::Error>> {
    let secret = env::var("JWT_REFRESH_SECRET").map_err(|e| {
        log::error!("{}", e);
        Box::new(e)
    })?;

    let token_data = _decode_jwt(token, &secret)?;

    Ok(token_data)
}

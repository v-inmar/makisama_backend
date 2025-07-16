use bcrypt::{BcryptError, DEFAULT_COST, hash, verify};

pub fn make_hash(value: &str) -> Result<String, BcryptError> {
    let hashed = hash(value, DEFAULT_COST)?;
    Ok(hashed)
}

pub fn is_matched(raw: &str, hashed: &str) -> Result<bool, BcryptError> {
    let result = verify(raw, hashed)?;
    Ok(result)
}

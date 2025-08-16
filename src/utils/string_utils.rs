use std::str::FromStr;

use email_address::EmailAddress;
use regex::Regex;

pub fn is_alphanumeric_or_underscore(s: &str) -> bool {
    s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

pub fn is_first_character_underscore(s: &str) -> bool {
    if let Some(c) = s.chars().next() {
        c == '_'
    } else {
        false
    }
}

/// Checks if the input string contains only alphabetic characters (A-Z, a-z).
///
/// This function uses a regular expression to validate if the input string
/// consists solely of alphabetic characters. The regular expression matches
/// strings containing only uppercase and lowercase letters. If the regex
/// fails to compile, an error message is logged, and the function returns `false`.
///
/// # Arguments
///
/// * `s` - A string slice that represents the input to check.
///
/// # Returns
///
/// * `true` if the string contains only alphabetic characters.
/// * `false` otherwise, or if regex compilation fails.
///
/// # Example
/// ```
/// assert!(is_alphabet_only("Hello"));
/// assert!(!is_alphabet_only("Hello123"));
/// assert!(!is_alphabet_only("Hello@World"));
/// ```
pub fn is_alphabet_only(s: &str) -> bool {
    let re = match Regex::new(r"^a-zA-Z+$") {
        Ok(regex) => regex,
        Err(_) => {
            log::error!("Failed to compile regex");
            return false;
        }
    };

    re.is_match(s)
}

/// Validates whether the given string is a valid email format.
///
/// This function attempts to parse the input string as an email address using
/// the `EmailAddress` type. If the parsing succeeds, it returns `true`; otherwise,
/// it returns `false`. This function does not validate whether the email address
/// actually exists, only if it follows the correct syntax.
///
/// # Arguments
///
/// * `email` - A string slice representing the email address to validate.
///
/// # Returns
///
/// * `true` if the email is in a valid format.
/// * `false` otherwise.
///
/// # Example
/// ```
/// assert!(is_email_format("example@example.com"));
/// assert!(!is_email_format("not-an-email"));
/// ```
pub fn is_email_format(email: &str) -> bool {
    return EmailAddress::from_str(email).is_ok();
}

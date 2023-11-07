use crate::telemetry;

use super::error::Error;

#[derive(Clone, Debug, sqlx::Type)]
#[sqlx(transparent)]
#[cfg_attr(test, derive(fake::Dummy))]
pub struct Email(
    #[cfg_attr(test, dummy(faker = "fake::faker::internet::en::SafeEmail()"))]
    String,
);

//TODO tests
impl TryFrom<String> for Email {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if validate_email(value.as_str()) {
            Ok(Self(value))
        } else {
            Err(Error::Validation("unsupported email address"))
                .map_err(telemetry::warn)
        }
    }
}

fn validate_email(email: &str) -> bool {
    if email.len() > 50 {
        return false;
    }
    let Some((username, domain)) = email.split_once('@') else {
        return false;
    };
    if !domain.contains('.') {
        return false;
    }
    for part in [username, domain] {
        let is_invalid_character = |c| match c {
            c if c.is_alphanumeric() => false,
            '.' | '-' | '_' => false,
            _ => true,
        };
        if part.contains("..") || part.chars().any(is_invalid_character) {
            return false;
        }
    }
    true
}

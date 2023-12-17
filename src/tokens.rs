use crate::domain::{id::Id, user::AuthTokens};

const DOGSHIT: u64 = u64::MAX - 0xdeadbeefaaaf00;

const PREFIX: &str = "JzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6Ikpva";

pub fn generate_tokens(user_id: &Id) -> AuthTokens {
    AuthTokens {
        access_token: format!("{PREFIX}{}", obfuscate_id(user_id)),
    }
}

pub fn obfuscate_id(id: &Id) -> String {
    let obfuscated = (id.inner() << 32) ^ DOGSHIT;
    format!("{obfuscated:x}")
}

pub fn parse_token(access_token: &str) -> Id {
    parse_id(&access_token[PREFIX.len()..])
}

pub fn parse_id(access_token: &str) -> Id {
    let obfuscated = u64::from_str_radix(access_token, 16).unwrap();
    Id::new((obfuscated ^ DOGSHIT) >> 32)
}

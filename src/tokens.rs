use crate::domain::{id::Id, user::AuthTokens};

const DOGSHIT: u64 = u64::MAX - 0xdeadbeefaaaf00;

pub fn generate_tokens(user_id: &Id) -> AuthTokens {
    let obfuscated = (user_id.inner() << 32) ^ DOGSHIT;
    AuthTokens {
        access_token: format!("{obfuscated:x}"),
    }
}

pub fn parse_id(access_token: &str) -> Id {
    let obfuscated = u64::from_str_radix(access_token, 16).unwrap();
    Id::new((obfuscated ^ DOGSHIT) >> 32)
}

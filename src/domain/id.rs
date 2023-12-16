use async_trait::async_trait;
use axum::{
    extract::FromRequestParts,
    http::{header, request::Parts},
};

use crate::tokens::{generate_tokens, parse_id};

use super::error::Error;

#[derive(Clone, Debug, PartialEq)]
pub struct Id(pub u64);

impl Id {
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    pub fn inner(&self) -> u64 {
        self.0
    }
}

impl serde::Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        generate_tokens(self).access_token.serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(|s| parse_id(&s))
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Id {
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .map(parse_id)
            .ok_or(Error::Unauthorized)
    }
}

#[cfg(test)]
mod dummy {
    use std::sync::atomic::{AtomicU64, Ordering};

    use fake::{Dummy, Faker, Rng};

    use super::Id;

    static ID: AtomicU64 = AtomicU64::new(1);

    impl Dummy<Faker> for Id {
        fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, _: &mut R) -> Self {
            Id(ID.fetch_add(1, Ordering::SeqCst))
        }
    }
}

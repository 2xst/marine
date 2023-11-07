use uuid::{fmt::Simple, Uuid};

use super::sensitive::{Sensitive, Zeroizable};

pub type Token = Sensitive<Zeroizable<Simple>>;

impl Token {
    pub fn generate() -> Self {
        Self::new(Zeroizable(Uuid::new_v4().simple()))
    }
}

#[cfg(test)]
impl fake::Dummy<fake::Faker> for Token {
    fn dummy_with_rng<R: fake::Rng + ?Sized>(
        _: &fake::Faker,
        _: &mut R,
    ) -> Self {
        Self::generate()
    }
}

#[derive(Clone, Debug, sqlx::Type)]
#[sqlx(transparent)]
pub struct Id(u64);

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

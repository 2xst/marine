mod users;

use crate::ports::{database::Database, hash::Hasher};

#[derive(Clone, Default)]
pub struct App<DB, H>
where
    DB: Database,
    H: Hasher,
{
    database: DB,
    hasher: H,
}

impl<DB, H> App<DB, H>
where
    DB: Database,
    H: Hasher,
{
    pub fn with(database: DB, hasher: H) -> Self {
        Self { database, hasher }
    }
}

#[cfg(test)]
use crate::adapters::mocks::{database::MockDatabase, hash::MockHasher};

#[cfg(test)]
#[allow(unused)]
type TestApp = App<MockDatabase, MockHasher>;

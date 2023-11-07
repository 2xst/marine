use async_trait::async_trait;

use crate::{
    domain::{error::Result, user::NewUser},
    ports::database::{Database, UserDatabase},
};

mockall::mock! {
    pub Database {}

    #[async_trait]
    impl UserDatabase for Database {
        async fn create_user(&mut self, user: &NewUser) -> Result<()>;
    }

    impl Database for Database {}
}

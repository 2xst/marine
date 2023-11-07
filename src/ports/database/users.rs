use async_trait::async_trait;

use crate::domain::{error::Result, user::NewUser};

#[async_trait]
pub trait UserDatabase {
    async fn create_user(&mut self, user: &NewUser) -> Result<()>;
}

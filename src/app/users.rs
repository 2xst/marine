use crate::domain::{
    error::Result,
    user::{NewUser, NewUserRequest},
};

use super::App;

impl App {
    #[tracing::instrument(skip(self))]
    pub async fn signup(&mut self, req: NewUserRequest) -> Result<()> {
        let password_hash = self.hasher.hash_password(req.password.try_into()?)?;
        let user = NewUser {
            email: req.email.try_into()?,
            password_hash,
        };
        self.database.insert_user(&user).await
    }
}

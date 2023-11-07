use crate::domain::{
    error::Result,
    token::Token,
    user::{NewUser, NewUserRequest},
};

use super::App;

impl App {
    #[tracing::instrument(skip(self))]
    pub async fn signup(&mut self, req: NewUserRequest) -> Result<()> {
        let password_hash =
            self.hasher.hash_password(req.password.try_into()?)?;
        let user = NewUser {
            email: req.email.try_into()?,
            password_hash,
            verification_token: Token::generate(),
            verified: false,
            refresh_token: Token::generate(),
        };
        self.database.create_user(&user).await
    }
}

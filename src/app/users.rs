use crate::{
    domain::{
        error::Result,
        id::Id,
        password::Password,
        user::{AuthTokens, LoginRequest, NewUser, NewUserRequest},
    },
    tokens::generate_tokens,
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

    #[tracing::instrument(skip(self))]
    pub async fn login(&mut self, req: LoginRequest) -> Result<AuthTokens> {
        let user = self.database.find_user(&req.ident).await?;
        let password = Password::new_unchecked(req.password);
        self.hasher.verify_password(password, user.password_hash)?;
        Ok(generate_tokens(&user.id))
    }

    #[tracing::instrument(skip(self))]
    pub async fn update_credentials(&mut self, id: &Id, req: NewUserRequest) -> Result<()> {
        let password_hash = self.hasher.hash_password(req.password.try_into()?)?;
        let user = NewUser {
            email: req.email.try_into()?,
            password_hash,
        };
        self.database.update_user(id, &user).await
    }
}

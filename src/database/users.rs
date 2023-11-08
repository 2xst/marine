use crate::domain::{error::Result, user::NewUser};

use super::Database;

impl Database {
    #[tracing::instrument(skip(self))]
    pub async fn create_user(&mut self, user: &NewUser) -> Result<()> {
        todo!()
    }
}

// #[cfg(test)]
// #[cfg(not(feature = "skip-io-tests"))]
// mod tests {
//     use fake::{Fake, Faker};
//
//     use crate::telemetry::init_test_telemetry;
//
//     use super::{Error, Database, NewUser};
//
//     #[tokio::test]
//     async fn reject_duplicate_email() {
//         init_test_telemetry();
//         let mut db = todo!();
//         let user = Faker.fake();
//         let res = db.create_user(&user).await;
//         assert!(res.is_ok());
//         let user = NewUser {
//             email: user.email,
//             ..Faker.fake()
//         };
//         let res = db.create_user(&user).await;
//         assert!(res.is_err_and(|e| matches!(e, Error::EmailTaken)));
//     }
// }

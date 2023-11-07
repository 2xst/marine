mod users;

pub use users::UserDatabase;

pub trait Database: UserDatabase {}

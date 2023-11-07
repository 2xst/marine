use std::fmt;

use secrecy::{
    zeroize::DefaultIsZeroes, CloneableSecret, DebugSecret, ExposeSecret,
    Secret, Zeroize,
};

pub struct Sensitive<T: Zeroize>(Secret<T>);

#[derive(Clone, Copy, Default)]
pub struct Zeroizable<T: Clone + Copy + Default>(pub T);

impl<T: Zeroize> Sensitive<T> {
    pub fn new(value: T) -> Self {
        Self(Secret::new(value))
    }
}

impl<T: Zeroize> From<T> for Sensitive<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Zeroize + CloneableSecret> Clone for Sensitive<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: DebugSecret + Zeroize> fmt::Debug for Sensitive<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<T: Zeroize> ExposeSecret<T> for Sensitive<T> {
    fn expose_secret(&self) -> &T {
        self.0.expose_secret()
    }
}

impl<T: Clone + Copy + Default> CloneableSecret for Zeroizable<T> {}
impl<T: Clone + Copy + Default> DefaultIsZeroes for Zeroizable<T> {}
impl<T: Clone + Copy + Default> DebugSecret for Zeroizable<T> {}

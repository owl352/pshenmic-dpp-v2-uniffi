#[derive(Debug)]
pub enum LockError {
    FailedToLock,
}

#[derive(Debug)]
pub enum IdentityError {
    FailedCreateIdentity,
}

impl std::fmt::Display for LockError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl std::fmt::Display for IdentityError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

uniffi::include_scaffolding!("errors");


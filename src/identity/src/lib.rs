use errors::IdentityError;
use enums::PlatformVersionFFI;
use errors::LockError;
use identifier::IdentifierFFI;
use pshenmic_dpp_v2::identity::IdentityBind;
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct IdentityFFI(Arc<RwLock<IdentityBind>>);

impl IdentityFFI {
    pub fn new(
        id: Arc<IdentifierFFI>,
        platform_version_bind: Option<PlatformVersionFFI>,
    ) -> Result<Self, IdentityError> {
        Ok(Self(Arc::new(RwLock::new(
            IdentityBind::new(
                id.as_ref().clone().into(),
                platform_version_bind.map(|v| v.into()),
            )
            .map_err(|_| errors::IdentityError::FailedCreateIdentity)?,
        ))))
    }

    pub fn set_balance(&self, balance: u64) -> Result<(), LockError> {
        self.0
            .write()
            .map_err(|_| LockError::FailedToLock)?
            .set_balance(balance);

        Ok(())
    }

    pub fn get_balance(&self) -> u64 {
        self.0.read().unwrap().get_balance()
    }
}

uniffi::include_scaffolding!("identity");

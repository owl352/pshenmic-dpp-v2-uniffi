use pshenmic_dpp_v2::identifier::IdentifierBind;

#[derive(Debug, thiserror::Error)]
pub enum IdentifierError {
    #[error("ParsingErrorBase58")]
    ParsingErrorBase58,
}

#[derive(Clone)]
pub struct Identifier(IdentifierBind);

impl Identifier {
    pub fn from_base58(id: String) -> Result<Identifier, IdentifierError> {
        Ok(Identifier(
            IdentifierBind::from_base58(id).map_err(|_| IdentifierError::ParsingErrorBase58)?,
        ))
    }
    
    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }
}

uniffi::include_scaffolding!("identifier");

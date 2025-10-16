use pshenmic_dpp_v2::enums::encoding::EncodingBind;
use pshenmic_dpp_v2::identifier::IdentifierBind;
use std::fmt;
use std::sync::Arc;

#[derive(Debug)]
pub enum IdentifierErrorFFI {
    ParsingErrorBase58 { err: String },
    ParsingErrorBytes { err: String },
}

impl fmt::Display for IdentifierErrorFFI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum DynamicId {
    Str { id: String },
    Bytes { id: Vec<u8> },
    FfiId { id: Arc<IdentifierFFI> },
}

#[derive(Clone, Debug)]
pub struct IdentifierFFI(IdentifierBind);

impl From<IdentifierBind> for IdentifierFFI {
    fn from(bind: IdentifierBind) -> Self {
        Self(bind)
    }
}

impl From<IdentifierFFI> for IdentifierBind {
    fn from(id: IdentifierFFI) -> Self {
        id.0
    }
}

impl IdentifierFFI {
    pub fn new(id: DynamicId) -> Result<Self, IdentifierErrorFFI> {
        match id {
            DynamicId::Str { id } => match IdentifierBind::from_string(id, EncodingBind::Base58) {
                Ok(identifier) => Ok(Self(identifier)),
                Err(e) => Err(IdentifierErrorFFI::ParsingErrorBase58 { err: e.to_string() }),
            },
            DynamicId::Bytes { id } => match IdentifierBind::from_vec(id) {
                Ok(identifier) => Ok(Self(identifier)),
                Err(e) => Err(IdentifierErrorFFI::ParsingErrorBytes { err: e.to_string() }),
            },
            DynamicId::FfiId { id } => Ok(id.as_ref().clone()),
        }
    }

    pub fn to_hex(&self) -> String {
        self.0.to_string(EncodingBind::Hex)
    }

    pub fn to_base58(&self) -> String {
        self.0.to_string(EncodingBind::Base58)
    }

    pub fn to_base64(&self) -> String {
        self.0.to_string(EncodingBind::Base64)
    }
}

uniffi::include_scaffolding!("identifier");

use pshenmic_dpp_v2::enums::encoding::EncodingBind;
use pshenmic_dpp_v2::errors::identifier::IdentifierError;
use pshenmic_dpp_v2::identifier::IdentifierBind;

#[derive(Debug)]
pub enum IdentifierErrorFFI {
    ParsingErrorBase58(String),
    ParsingErrorBytes(String),
}

pub enum DynamicId {
    Str(String),
    Bytes(Vec<u8>),
    FFI(IdentifierFFI),
}

#[derive(Clone)]
pub struct IdentifierFFI(IdentifierBind);

impl IdentifierFFI {
    pub fn new(id: DynamicId) -> Result<Self, IdentifierErrorFFI> {
        match id {
            DynamicId::Str(s) => match IdentifierBind::from_string(s, EncodingBind::Base58) {
                Ok(identifier) => Ok(Self(identifier)),
                Err(e) => Err(IdentifierErrorFFI::ParsingErrorBase58(e.to_string())),
            },
            DynamicId::Bytes(bytes) => match IdentifierBind::from_vec(bytes) {
                Ok(identifier) => Ok(Self(identifier)),
                Err(err) => Err(IdentifierErrorFFI::ParsingErrorBytes(err.to_string())),
            },
            DynamicId::FFI(identifier) => Ok(identifier),
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

uniffi::include_scaffolding!("pshenmic-dpp");

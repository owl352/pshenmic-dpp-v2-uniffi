use pshenmic_dpp_v2::enums::platform_version::PlatformVersionBind;

#[derive(uniffi::Enum)]
#[allow(non_camel_case_types)]
pub enum PlatformVersionFFI {
    PLATFORM_V1,
    PLATFORM_V2,
    PLATFORM_V3,
    PLATFORM_V4,
    PLATFORM_V5,
    PLATFORM_V6,
    PLATFORM_V7,
    PLATFORM_V8,
    PLATFORM_V9
}

impl From<PlatformVersionBind> for PlatformVersionFFI {
    fn from(value: PlatformVersionBind) -> Self {
        match value {
            PlatformVersionBind::PLATFORM_V1 => Self::PLATFORM_V1,
            PlatformVersionBind::PLATFORM_V2 => Self::PLATFORM_V2,
            PlatformVersionBind::PLATFORM_V3 => Self::PLATFORM_V3,
            PlatformVersionBind::PLATFORM_V4 => Self::PLATFORM_V4,
            PlatformVersionBind::PLATFORM_V5 => Self::PLATFORM_V5,
            PlatformVersionBind::PLATFORM_V6 => Self::PLATFORM_V6,
            PlatformVersionBind::PLATFORM_V7 => Self::PLATFORM_V7,
            PlatformVersionBind::PLATFORM_V8 => Self::PLATFORM_V8,
            PlatformVersionBind::PLATFORM_V9 => Self::PLATFORM_V9,
        }
    }
} 

impl From<PlatformVersionFFI> for PlatformVersionBind {
    fn from(value: PlatformVersionFFI) -> Self {
        match value { 
            PlatformVersionFFI::PLATFORM_V1 => Self::PLATFORM_V1,
            PlatformVersionFFI::PLATFORM_V2 => Self::PLATFORM_V2,
            PlatformVersionFFI::PLATFORM_V3 => Self::PLATFORM_V3,
            PlatformVersionFFI::PLATFORM_V4 => Self::PLATFORM_V4,
            PlatformVersionFFI::PLATFORM_V5 => Self::PLATFORM_V5,
            PlatformVersionFFI::PLATFORM_V6 => Self::PLATFORM_V6,
            PlatformVersionFFI::PLATFORM_V7 => Self::PLATFORM_V7,
            PlatformVersionFFI::PLATFORM_V8 => Self::PLATFORM_V8,
            PlatformVersionFFI::PLATFORM_V9 => Self::PLATFORM_V9,
        }
    }
}

uniffi::setup_scaffolding!();
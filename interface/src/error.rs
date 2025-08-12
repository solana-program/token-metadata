//! Interface error types

use solana_program_error::{ProgramError, ToStr};

/// Errors that may be returned by the interface.
#[repr(u32)]
#[derive(Clone, Debug, Eq, thiserror::Error, num_derive::FromPrimitive, PartialEq)]
pub enum TokenMetadataError {
    /// Incorrect account provided
    #[error("Incorrect account provided")]
    IncorrectAccount = 901_952_957,
    /// Mint has no mint authority
    #[error("Mint has no mint authority")]
    MintHasNoMintAuthority,
    /// Incorrect mint authority has signed the instruction
    #[error("Incorrect mint authority has signed the instruction")]
    IncorrectMintAuthority,
    /// Incorrect metadata update authority has signed the instruction
    #[error("Incorrect metadata update authority has signed the instruction")]
    IncorrectUpdateAuthority,
    /// Token metadata has no update authority
    #[error("Token metadata has no update authority")]
    ImmutableMetadata,
    /// Key not found in metadata account
    #[error("Key not found in metadata account")]
    KeyNotFound,
}

impl From<TokenMetadataError> for ProgramError {
    fn from(e: TokenMetadataError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl ToStr for TokenMetadataError {
    fn to_str(&self) -> &'static str {
        match self {
            TokenMetadataError::IncorrectAccount => "Incorrect account provided",
            TokenMetadataError::MintHasNoMintAuthority => "Mint has no mint authority",
            TokenMetadataError::IncorrectMintAuthority => {
                "Incorrect mint authority has signed the instruction"
            }
            TokenMetadataError::IncorrectUpdateAuthority => {
                "Incorrect metadata update authority has signed the instruction"
            }
            TokenMetadataError::ImmutableMetadata => "Token metadata has no update authority",
            TokenMetadataError::KeyNotFound => "Key not found in metadata account",
        }
    }
}

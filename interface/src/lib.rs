//! Crate defining an interface for token-metadata

#![allow(clippy::arithmetic_side_effects)]
#![no_std]
#![deny(missing_docs)]
#![cfg_attr(not(test), forbid(unsafe_code))]

extern crate alloc;

pub mod error;
pub mod instruction;
pub mod state;

// Export current sdk types for downstream users building with a different sdk
// version Export borsh for downstream users
pub use {
    borsh, solana_address, solana_borsh, solana_instruction, solana_nullable, solana_program_error,
};

/// Namespace for all programs implementing token-metadata
pub const NAMESPACE: &str = "spl_token_metadata_interface";

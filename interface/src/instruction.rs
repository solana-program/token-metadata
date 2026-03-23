//! Instruction types

use {
    crate::state::Field,
    alloc::{string::String, vec, vec::Vec},
    borsh::{BorshDeserialize, BorshSerialize},
    solana_address::Address,
    solana_instruction::{account_meta::AccountMeta, Instruction},
    solana_nullable::MaybeNull,
    solana_program_error::ProgramError,
    spl_discriminator::{discriminator::ArrayDiscriminator, SplDiscriminate},
};

#[cfg(feature = "serde-traits")]
use serde::{Deserialize, Serialize};

/// Initialization instruction data
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-traits", serde(rename_all = "camelCase"))]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, SplDiscriminate)]
#[discriminator_hash_input("spl_token_metadata_interface:initialize_account")]
pub struct Initialize {
    /// Longer name of the token
    pub name: String,
    /// Shortened symbol of the token
    pub symbol: String,
    /// URI pointing to more metadata (image, video, etc.)
    pub uri: String,
}

/// Update field instruction data
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-traits", serde(rename_all = "camelCase"))]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, SplDiscriminate)]
#[discriminator_hash_input("spl_token_metadata_interface:updating_field")]
pub struct UpdateField {
    /// Field to update in the metadata
    pub field: Field,
    /// Value to write for the field
    pub value: String,
}

/// Remove key instruction data
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-traits", serde(rename_all = "camelCase"))]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, SplDiscriminate)]
#[discriminator_hash_input("spl_token_metadata_interface:remove_key_ix")]
pub struct RemoveKey {
    /// If the idempotent flag is set to true, then the instruction will not
    /// error if the key does not exist
    pub idempotent: bool,
    /// Key to remove in the additional metadata portion
    pub key: String,
}

/// Update authority instruction data
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, SplDiscriminate)]
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-traits", serde(rename_all = "camelCase"))]
#[discriminator_hash_input("spl_token_metadata_interface:update_the_authority")]
pub struct UpdateAuthority {
    /// New authority for the token metadata, or unset if `None`
    #[cfg_attr(feature = "serde-traits", serde(with = "maybe_null_address_str"))]
    pub new_authority: MaybeNull<Address>,
}

/// Instruction data for Emit
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-traits", serde(rename_all = "camelCase"))]
#[derive(Clone, Debug, PartialEq, BorshSerialize, BorshDeserialize, SplDiscriminate)]
#[discriminator_hash_input("spl_token_metadata_interface:emitter")]
pub struct Emit {
    /// Start of range of data to emit
    pub start: Option<u64>,
    /// End of range of data to emit
    pub end: Option<u64>,
}

/// All instructions that must be implemented in the token-metadata interface
#[cfg_attr(feature = "serde-traits", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde-traits", serde(rename_all = "camelCase"))]
#[derive(Clone, Debug, PartialEq)]
pub enum TokenMetadataInstruction {
    /// Initializes a TLV entry with the basic token-metadata fields.
    ///
    /// Assumes that the provided mint is an SPL token mint, that the metadata
    /// account is allocated and assigned to the program, and that the metadata
    /// account has enough lamports to cover the rent-exempt reserve.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[w]` Metadata
    ///   1. `[]` Update authority
    ///   2. `[]` Mint
    ///   3. `[s]` Mint authority
    ///
    /// Data: `Initialize` data, name / symbol / uri strings
    Initialize(Initialize),

    /// Updates a field in a token-metadata account.
    ///
    /// The field can be one of the required fields (name, symbol, URI), or a
    /// totally new field denoted by a "key" string.
    ///
    /// By the end of the instruction, the metadata account must be properly
    /// re-sized based on the new size of the TLV entry.
    ///   * If the new size is larger, the program must first reallocate to
    ///     avoid overwriting other TLV entries.
    ///   * If the new size is smaller, the program must reallocate at the end
    ///     so that it's possible to iterate over TLV entries
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[w]` Metadata account
    ///   1. `[s]` Update authority
    ///
    /// Data: `UpdateField` data, specifying the new field and value. If the
    /// field does not exist on the account, it will be created. If the
    /// field does exist, it will be overwritten.
    UpdateField(UpdateField),

    /// Removes a key-value pair in a token-metadata account.
    ///
    /// This only applies to additional fields, and not the base name / symbol /
    /// URI fields.
    ///
    /// By the end of the instruction, the metadata account must be properly
    /// re-sized at the end based on the new size of the TLV entry.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[w]` Metadata account
    ///   1. `[s]` Update authority
    ///
    /// Data: the string key to remove. If the idempotent flag is set to false,
    /// returns an error if the key is not present
    RemoveKey(RemoveKey),

    /// Updates the token-metadata authority
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[w]` Metadata account
    ///   1. `[s]` Current update authority
    ///
    /// Data: the new authority. Can be unset using a `None` value
    UpdateAuthority(UpdateAuthority),

    /// Emits the token-metadata as return data
    ///
    /// The format of the data emitted follows exactly the `TokenMetadata`
    /// struct, but it's possible that the account data is stored in another
    /// format by the program.
    ///
    /// With this instruction, a program that implements the token-metadata
    /// interface can return `TokenMetadata` without adhering to the specific
    /// byte layout of the `TokenMetadata` struct in any accounts.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[]` Metadata account
    Emit(Emit),
}
impl TokenMetadataInstruction {
    /// Unpacks a byte buffer into a
    /// [`TokenMetadataInstruction`](enum.TokenMetadataInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() < ArrayDiscriminator::LENGTH {
            return Err(ProgramError::InvalidInstructionData);
        }
        let (discriminator, rest) = input.split_at(ArrayDiscriminator::LENGTH);
        Ok(match discriminator {
            Initialize::SPL_DISCRIMINATOR_SLICE => {
                let data = Initialize::try_from_slice(rest)?;
                Self::Initialize(data)
            }
            UpdateField::SPL_DISCRIMINATOR_SLICE => {
                let data = UpdateField::try_from_slice(rest)?;
                Self::UpdateField(data)
            }
            RemoveKey::SPL_DISCRIMINATOR_SLICE => {
                let data = RemoveKey::try_from_slice(rest)?;
                Self::RemoveKey(data)
            }
            UpdateAuthority::SPL_DISCRIMINATOR_SLICE => {
                let data = UpdateAuthority::try_from_slice(rest)?;
                Self::UpdateAuthority(data)
            }
            Emit::SPL_DISCRIMINATOR_SLICE => {
                let data = Emit::try_from_slice(rest)?;
                Self::Emit(data)
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    /// Packs a [`TokenMetadataInstruction`](enum.TokenMetadataInstruction.html)
    /// into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = vec![];
        match self {
            Self::Initialize(data) => {
                buf.extend_from_slice(Initialize::SPL_DISCRIMINATOR_SLICE);
                buf.append(&mut borsh::to_vec(data).unwrap());
            }
            Self::UpdateField(data) => {
                buf.extend_from_slice(UpdateField::SPL_DISCRIMINATOR_SLICE);
                buf.append(&mut borsh::to_vec(data).unwrap());
            }
            Self::RemoveKey(data) => {
                buf.extend_from_slice(RemoveKey::SPL_DISCRIMINATOR_SLICE);
                buf.append(&mut borsh::to_vec(data).unwrap());
            }
            Self::UpdateAuthority(data) => {
                buf.extend_from_slice(UpdateAuthority::SPL_DISCRIMINATOR_SLICE);
                buf.append(&mut borsh::to_vec(data).unwrap());
            }
            Self::Emit(data) => {
                buf.extend_from_slice(Emit::SPL_DISCRIMINATOR_SLICE);
                buf.append(&mut borsh::to_vec(data).unwrap());
            }
        };
        buf
    }
}

/// Creates an `Initialize` instruction
#[allow(clippy::too_many_arguments)]
pub fn initialize(
    program_id: &Address,
    metadata: &Address,
    update_authority: &Address,
    mint: &Address,
    mint_authority: &Address,
    name: String,
    symbol: String,
    uri: String,
) -> Instruction {
    let data = TokenMetadataInstruction::Initialize(Initialize { name, symbol, uri });
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*metadata, false),
            AccountMeta::new_readonly(*update_authority, false),
            AccountMeta::new_readonly(*mint, false),
            AccountMeta::new_readonly(*mint_authority, true),
        ],
        data: data.pack(),
    }
}

/// Creates an `UpdateField` instruction
pub fn update_field(
    program_id: &Address,
    metadata: &Address,
    update_authority: &Address,
    field: Field,
    value: String,
) -> Instruction {
    let data = TokenMetadataInstruction::UpdateField(UpdateField { field, value });
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*metadata, false),
            AccountMeta::new_readonly(*update_authority, true),
        ],
        data: data.pack(),
    }
}

/// Creates a `RemoveKey` instruction
pub fn remove_key(
    program_id: &Address,
    metadata: &Address,
    update_authority: &Address,
    key: String,
    idempotent: bool,
) -> Instruction {
    let data = TokenMetadataInstruction::RemoveKey(RemoveKey { key, idempotent });
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*metadata, false),
            AccountMeta::new_readonly(*update_authority, true),
        ],
        data: data.pack(),
    }
}

/// Creates an `UpdateAuthority` instruction
pub fn update_authority(
    program_id: &Address,
    metadata: &Address,
    current_authority: &Address,
    new_authority: MaybeNull<Address>,
) -> Instruction {
    let data = TokenMetadataInstruction::UpdateAuthority(UpdateAuthority { new_authority });
    Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new(*metadata, false),
            AccountMeta::new_readonly(*current_authority, true),
        ],
        data: data.pack(),
    }
}

/// Creates an `Emit` instruction
pub fn emit(
    program_id: &Address,
    metadata: &Address,
    start: Option<u64>,
    end: Option<u64>,
) -> Instruction {
    let data = TokenMetadataInstruction::Emit(Emit { start, end });
    Instruction {
        program_id: *program_id,
        accounts: vec![AccountMeta::new_readonly(*metadata, false)],
        data: data.pack(),
    }
}

// Preserves the old serde behavior from before the  `OptionalNonZeroPubkey` -> `MaybeNull<Address>`
// migration. `Some(address)` serializes as a base58 string and `None` as `null`. Reference:
// https://github.com/solana-program/libraries/blob/8c73d863e928e726a555085fdb5e09a190df5786/pod/src/optional_keys.rs#L76-L129
#[cfg(feature = "serde-traits")]
mod maybe_null_address_str {
    use {
        alloc::string::{String, ToString},
        core::str::FromStr,
        serde::{Deserialize, Deserializer, Serializer},
        solana_address::Address,
        solana_nullable::MaybeNull,
    };

    pub fn serialize<S>(value: &MaybeNull<Address>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match Option::<Address>::from(*value) {
            Some(address) => serializer.serialize_str(&address.to_string()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<MaybeNull<Address>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Option::<String>::deserialize(deserializer)?;
        value
            .map(|value| Address::from_str(&value).map_err(serde::de::Error::custom))
            .transpose()?
            .try_into()
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod test {
    #[cfg(feature = "serde-traits")]
    use core::str::FromStr;
    use {
        super::*,
        crate::NAMESPACE,
        alloc::{format, string::ToString, vec},
        solana_sha256_hasher::hashv,
    };

    fn check_pack_unpack<T: BorshSerialize>(
        instruction: TokenMetadataInstruction,
        discriminator: &[u8],
        data: T,
    ) {
        let mut expect = vec![];
        expect.extend_from_slice(discriminator.as_ref());
        expect.append(&mut borsh::to_vec(&data).unwrap());
        let packed = instruction.pack();
        assert_eq!(packed, expect);
        let unpacked = TokenMetadataInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, instruction);
    }

    #[test]
    fn initialize_pack() {
        let name = "My test token";
        let symbol = "TEST";
        let uri = "http://test.test";
        let data = Initialize {
            name: name.to_string(),
            symbol: symbol.to_string(),
            uri: uri.to_string(),
        };
        let check = TokenMetadataInstruction::Initialize(data.clone());
        let preimage = hashv(&[format!("{NAMESPACE}:initialize_account").as_bytes()]);
        let discriminator = &preimage.as_ref()[..ArrayDiscriminator::LENGTH];
        check_pack_unpack(check, discriminator, data);
    }

    #[test]
    fn update_field_pack() {
        let field = "MyTestField";
        let value = "http://test.uri";
        let data = UpdateField {
            field: Field::Key(field.to_string()),
            value: value.to_string(),
        };
        let check = TokenMetadataInstruction::UpdateField(data.clone());
        let preimage = hashv(&[format!("{NAMESPACE}:updating_field").as_bytes()]);
        let discriminator = &preimage.as_ref()[..ArrayDiscriminator::LENGTH];
        check_pack_unpack(check, discriminator, data);
    }

    #[test]
    fn remove_key_pack() {
        let data = RemoveKey {
            key: "MyTestField".to_string(),
            idempotent: true,
        };
        let check = TokenMetadataInstruction::RemoveKey(data.clone());
        let preimage = hashv(&[format!("{NAMESPACE}:remove_key_ix").as_bytes()]);
        let discriminator = &preimage.as_ref()[..ArrayDiscriminator::LENGTH];
        check_pack_unpack(check, discriminator, data);
    }

    #[test]
    fn update_authority_pack() {
        let data = UpdateAuthority {
            new_authority: MaybeNull::default(),
        };
        let check = TokenMetadataInstruction::UpdateAuthority(data.clone());
        let preimage = hashv(&[format!("{NAMESPACE}:update_the_authority").as_bytes()]);
        let discriminator = &preimage.as_ref()[..ArrayDiscriminator::LENGTH];
        check_pack_unpack(check, discriminator, data);
    }

    #[test]
    fn emit_pack() {
        let data = Emit {
            start: None,
            end: Some(10),
        };
        let check = TokenMetadataInstruction::Emit(data.clone());
        let preimage = hashv(&[format!("{NAMESPACE}:emitter").as_bytes()]);
        let discriminator = &preimage.as_ref()[..ArrayDiscriminator::LENGTH];
        check_pack_unpack(check, discriminator, data);
    }

    #[cfg(feature = "serde-traits")]
    #[test]
    fn initialize_serde() {
        let data = Initialize {
            name: "Token Name".to_string(),
            symbol: "TST".to_string(),
            uri: "uri.test".to_string(),
        };
        let ix = TokenMetadataInstruction::Initialize(data);
        let serialized = serde_json::to_string(&ix).unwrap();
        let serialized_expected =
            "{\"initialize\":{\"name\":\"Token Name\",\"symbol\":\"TST\",\"uri\":\"uri.test\"}}";
        assert_eq!(&serialized, serialized_expected);

        let deserialized = serde_json::from_str::<TokenMetadataInstruction>(&serialized).unwrap();
        assert_eq!(ix, deserialized);
    }

    #[cfg(feature = "serde-traits")]
    #[test]
    fn update_field_serde() {
        let data = UpdateField {
            field: Field::Key("MyField".to_string()),
            value: "my field value".to_string(),
        };
        let ix = TokenMetadataInstruction::UpdateField(data);
        let serialized = serde_json::to_string(&ix).unwrap();
        let serialized_expected =
            "{\"updateField\":{\"field\":{\"key\":\"MyField\"},\"value\":\"my field value\"}}";
        assert_eq!(&serialized, serialized_expected);

        let deserialized = serde_json::from_str::<TokenMetadataInstruction>(&serialized).unwrap();
        assert_eq!(ix, deserialized);
    }

    #[cfg(feature = "serde-traits")]
    #[test]
    fn remove_key_serde() {
        let data = RemoveKey {
            key: "MyTestField".to_string(),
            idempotent: true,
        };
        let ix = TokenMetadataInstruction::RemoveKey(data);
        let serialized = serde_json::to_string(&ix).unwrap();
        let serialized_expected = "{\"removeKey\":{\"idempotent\":true,\"key\":\"MyTestField\"}}";
        assert_eq!(&serialized, serialized_expected);

        let deserialized = serde_json::from_str::<TokenMetadataInstruction>(&serialized).unwrap();
        assert_eq!(ix, deserialized);
    }

    #[cfg(feature = "serde-traits")]
    #[test]
    fn update_authority_serde() {
        let update_authority_option: Option<Address> =
            Some(Address::from_str("4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM").unwrap());
        let update_authority: MaybeNull<Address> = update_authority_option.try_into().unwrap();
        let data = UpdateAuthority {
            new_authority: update_authority,
        };
        let ix = TokenMetadataInstruction::UpdateAuthority(data);
        let serialized = serde_json::to_string(&ix).unwrap();
        let serialized_expected = "{\"updateAuthority\":{\"newAuthority\":\"4uQeVj5tqViQh7yWWGStvkEG1Zmhx6uasJtWCJziofM\"}}";
        assert_eq!(&serialized, serialized_expected);

        let deserialized = serde_json::from_str::<TokenMetadataInstruction>(&serialized).unwrap();
        assert_eq!(ix, deserialized);
    }

    #[cfg(feature = "serde-traits")]
    #[test]
    fn update_authority_serde_with_none() {
        let data = UpdateAuthority {
            new_authority: MaybeNull::default(),
        };
        let ix = TokenMetadataInstruction::UpdateAuthority(data);
        let serialized = serde_json::to_string(&ix).unwrap();
        let serialized_expected = "{\"updateAuthority\":{\"newAuthority\":null}}";
        assert_eq!(&serialized, serialized_expected);

        let deserialized = serde_json::from_str::<TokenMetadataInstruction>(&serialized).unwrap();
        assert_eq!(ix, deserialized);
    }

    #[cfg(feature = "serde-traits")]
    #[test]
    fn emit_serde() {
        let data = Emit {
            start: None,
            end: Some(10),
        };
        let ix = TokenMetadataInstruction::Emit(data);
        let serialized = serde_json::to_string(&ix).unwrap();
        let serialized_expected = "{\"emit\":{\"start\":null,\"end\":10}}";
        assert_eq!(&serialized, serialized_expected);

        let deserialized = serde_json::from_str::<TokenMetadataInstruction>(&serialized).unwrap();
        assert_eq!(ix, deserialized);
    }
}

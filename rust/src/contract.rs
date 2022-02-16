use near_contract_standards::non_fungible_token::metadata::TokenMetadata;
use near_contract_standards::non_fungible_token::{NonFungibleToken, Token, TokenId};
use near_contract_standards::{impl_non_fungible_token_core, impl_non_fungible_token_enumeration, impl_non_fungible_token_approval};

use crate::*;

#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    NON_FUNGIBLE_TOKEN,
    TOKEN_METADATA,
    TOKEN_ENUMERATION,
    TOKEN_APPROVAL,
}

#[near_bindgen]
#[derive(PanicOnDefault, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub tokens: NonFungibleToken,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        Self {
            tokens: NonFungibleToken::new(
                StorageKey::NON_FUNGIBLE_TOKEN,
                "hatchet.testnet".parse().unwrap(),
                Some(StorageKey::TOKEN_METADATA),
                Some(StorageKey::TOKEN_ENUMERATION),
                Some(StorageKey::TOKEN_APPROVAL),
            ),
        }
    }

    pub fn mint(&mut self, token_id: TokenId) {
        self.tokens.internal_mint(
            token_id,
            env::predecessor_account_id(),
            Some(TokenMetadata {
                title: Some("My special nft token".to_string()),          // ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
                description: None,    // free-form description
                media: None, // URL to associated media, preferably to decentralized, content-addressed storage
                media_hash: None, // Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media` is included.
                copies: None, // number of copies of this set of metadata in existence when token was minted.
                issued_at: None, // ISO 8601 datetime when token was issued or minted
                expires_at: None, // ISO 8601 datetime when token expires
                starts_at: None, // ISO 8601 datetime when token starts being valid
                updated_at: None, // ISO 8601 datetime when token was last updated
                extra: None, // anything extra the NFT wants to store on-chain. Can be stringified JSON.
                reference: None, // URL to an off-chain JSON file with more info.
                reference_hash: None, // Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is included.
            }),
        );
    }
}

impl_non_fungible_token_core!(Contract, tokens);
impl_non_fungible_token_enumeration!(Contract, tokens);
impl_non_fungible_token_approval!(Contract, tokens);

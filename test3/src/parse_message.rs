// This contract will return the address from which it was deployed
const REQUIRED_MESSAGE_LENGTH: usize = 104;
const SIGNATURE_LENGTH: usize = 65;
use pwasm_std::{
    keccak,
    types::{Address, H256, U256},
    Vec,
};
use alloc::BTreeSet;
use super::bridge_validators::IBridgeValidators;
pub struct ParsedMessage {
    recipient: Address,
    amount: U256,
    tx_hash: H256,
    contract_address: Address,
}

pub fn parse_message(message: &[u8]) -> ParsedMessage {
    assert_eq!(message.len(), REQUIRED_MESSAGE_LENGTH, "Invalid message length (must be 104 bytes)");
    ParsedMessage {
        recipient: Address::from_slice(&message[..20]),
        amount: U256::from_little_endian(&message[20..52]),
        tx_hash: H256::from_slice(&message[52..84]),
        contract_address: Address::from_slice(&message[84..104]),
    }
}

fn ecrecover(hash: H256, byte: u8, h1: H256, h2: H256) -> ! {
    unimplemented!()
}

pub fn recover_address_from_signed_message(signature: &[u8], message: &[u8]) -> Address {
    assert_eq!(signature.len(), 65);
    ecrecover(
        hash_message(message),
        signature[64],
        H256::from_slice(&signature[..0x20]),
        H256::from_slice(&signature[0x20..0x40]),
    )
}

fn has_enough_valid_signatures(message: &[u8], vs: &[u8], rs: &[H256], ss: &[H256], validator_contract: &dyn IBridgeValidators) {
    assert_eq!(message.len(), REQUIRED_MESSAGE_LENGTH, "Invalid message length (must be 104 bytes)");
    let required_signatures = validator_contract.required_signatures();
    assert!(vs.len() >= required_signatures, "Not enough signatures");
    let hash = hash_message(message);
    let encountered_addresses = Vec::with_capacity(required_signatures);
    for i in 0..required_signatures {
        let recovered_address = ecrecover(hash, vs[i], rs[i], h2: H256)
    }
}

fn hash_message(message: &[u8]) -> H256 {
    const PREFIX: &[u8] = b"\x19Ethereum Signed Message:\n104";
    let mut v = Vec::with_capacity(PREFIX.len() + message.len());
    v.extend_from_slice(PREFIX);
    v.extend_from_slice(message);
    keccak(&v)
}

// Copyright 2018 POA Networks Ltd.
//
// This file is part of the POA Networks bridge contracts.
//
// The POA Networks bridge contracts are free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the License,
// or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License along with
// this program.  If not, see <https://www.gnu.org/licenses/>.
pub const REQUIRED_MESSAGE_LENGTH: usize = 104;
pub const SIGNATURE_LENGTH: usize = 65;
use super::bridge_validators::IBridgeValidators;
use crate::alloc::collections::BTreeSet;
use pwasm_std::{
    keccak,
    types::{Address, H256, U256},
    Vec,
};
pub struct ParsedMessage {
    recipient: Address,
    amount: U256,
    tx_hash: H256,
    contract_address: Address,
}

pub fn parse_message(message: &[u8]) -> ParsedMessage {
    assert_eq!(
        message.len(),
        REQUIRED_MESSAGE_LENGTH,
        "Invalid message length (must be 104 bytes)"
    );
    ParsedMessage {
        recipient: Address::from_slice(&message[..20]),
        amount: U256::from_little_endian(&message[20..52]),
        tx_hash: H256::from_slice(&message[52..84]),
        contract_address: Address::from_slice(&message[84..104]),
    }
}

fn ecrecover(_hash: H256, _byte: u8, _h1: H256, _h2: H256) -> Address {
    unimplemented!()
}

pub fn recover_address_from_signed_message(signature: &[u8], message: &[u8]) -> Address {
    assert_eq!(
        message.len(),
        REQUIRED_MESSAGE_LENGTH,
        "Invalid message length (must be 104 bytes)"
    );
    assert_eq!(signature.len(), 65);
    ecrecover(
        hash_message(message),
        signature[64],
        H256::from_slice(&signature[..0x20]),
        H256::from_slice(&signature[0x20..0x40]),
    )
}

fn has_enough_valid_signatures(
    message: &[u8],
    vs: &[u8],
    rs: &[H256],
    ss: &[H256],
    validator_contract: &dyn IBridgeValidators,
) {
    assert_eq!(
        message.len(),
        REQUIRED_MESSAGE_LENGTH,
        "Invalid message length (must be 104 bytes)"
    );
    let required_signatures = validator_contract.required_signatures();
    assert!(vs.len() >= required_signatures, "Not enough signatures");
    let hash = hash_message(message);
    let mut encountered_addresses: BTreeSet<Address> = BTreeSet::new();
    for i in 0..required_signatures {
        let recovered_address = ecrecover(hash, vs[i], rs[i], ss[i]);
        assert!(
            validator_contract.is_validator(recovered_address),
            "Signature from non-validator"
        );
        assert!(
            !encountered_addresses.contains(&recovered_address),
            "Duplicate signature"
        );
        encountered_addresses.insert(recovered_address);
    }
}

fn hash_message(message: &[u8]) -> H256 {
    const PREFIX: &[u8] = b"\x19Ethereum Signed Message:\n104";
    let mut v = Vec::with_capacity(PREFIX.len() + message.len());
    v.extend_from_slice(PREFIX);
    v.extend_from_slice(message);
    keccak(&v)
}

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
#![allow(non_snake_case)]
#![forbid(warnings)]
#![deny(unsafe_code)]
extern crate pwasm_abi;
extern crate pwasm_abi_derive;
extern crate pwasm_ethereum;
extern crate pwasm_std;
use super::super::parse_message;
pub use super::Validateable::Validateable;
use pwasm_abi::types::{Address, Vec, U256};
use pwasm_abi_derive::eth_abi;
use pwasm_std::keccak;

#[eth_abi(BasicHomeBridge)]
pub trait BasicHomeBridgeInterface {
    #[event]
    fn UserRequestForSignature(recipient: Address, value: U256);

    #[event]
    fn AffirmationCompleted(recipient: Address, value: U256, transactionHash: [u8; 32]);

    #[event]
    fn SignedForUserRequest(indexed_signer: Address, messageHash: [u8; 32]);

    #[event]
    fn SignedForAffirmation(indexed_signer: Address, transactionHash: [u8; 32]);

    #[event]
    fn CollectedSignatures(
        authorityResponsibleForRelay: Address,
        messageHash: [u8; 32],
        NumberOfCollectedSignatures: U256,
    );

    fn executeAffirmation(&mut self, recipient: Address, value: U256, transaction_hash: [u8; 32]);

    fn submitSignature(&mut self, signature: Vec<u8>, message: Vec<u8>);

    #[constant]
    fn numAffirmationsSigned(&self, withdrawl: [u8; 32]) -> U256;

    #[constant]
    fn affirmationsSigned(&self, withdrawal: [u8; 32]) -> bool;

    #[constant]
    fn signature(&self, hash: [u8; 32], index: U256) -> Vec<u8>;

    #[constant]
    fn messagesSigned(&self, message: [u8; 32]) -> bool;

    #[constant]
    fn message(&self, hash: [u8; 32]) -> Vec<u8>;

    #[constant]
    fn isAlreadyProcessed(&self, number: U256) -> bool;

    #[constant]
    fn numMessagesSigned(&self, message: [u8; 32]) -> U256;

    #[constant]
    fn requiredMessageLength(&self) -> U256;
}

pub struct BasicHomeBridgeContract(Validateable);

impl BasicHomeBridgeContract {
    fn setNumMessagesSigned(&mut self, _message: [u8; 32], _number: U256) {
        unimplemented!()
    }

    fn markAsProcessed(&self, v: U256) -> U256 {
        v | U256::from(1) << 255
    }

    fn messages(&self, _hash: [u8; 32]) -> Vec<u8> {
        unimplemented!()
    }

    fn signatures(&self, _hash: [u8; 32]) -> Vec<u8> {
        unimplemented!()
    }

    fn setSignatures(&mut self, _hash: [u8; 32], _signature: Vec<u8>) {
        unimplemented!()
    }

    fn setMessages(&mut self, _hash: [u8; 32], _message: Vec<u8>) {
        unimplemented!()
    }

    fn setAffirmationsSigned(&mut self, _withdrawl: [u8; 32], _status: bool) {
        unimplemented!()
    }

    fn setNumAffirmationsSigned(&mut self, _withdrawal: [u8; 32], _number: U256) {
        unimplemented!()
    }

    fn setMessagesSigned(&mut self, _hash: [u8; 32], _status: bool) {
        panic!("Don’t know how to store data persistently");
    }

    fn onExecuteAffirmation(&mut self, _address: Address, _value: U256) -> bool {
        unimplemented!()
    }
}

impl BasicHomeBridgeInterface for BasicHomeBridgeContract {
    #[allow(unsafe_code)]
    fn executeAffirmation(&mut self, recipient: Address, value: U256, transaction_hash: [u8; 32]) {
        self.0.check_validator();
        let (hash_msg, hash_sender) = {
            let mut buf = Vec::with_capacity(84);

            buf.extend_from_slice(recipient.as_ref());
            {
                let mut q = [0; 32];
                value.to_little_endian(&mut q);
                buf.extend_from_slice(&q);
            }

            buf.extend_from_slice(&transaction_hash);
            debug_assert_eq!(buf.len(), 84);
            let hash_msg: [u8; 32] = keccak(&buf).into();
            unsafe { buf.set_len(0) };
            let sender: Address = pwasm_ethereum::sender();
            buf.extend_from_slice(sender.as_ref());
            buf.extend_from_slice(hash_msg.as_ref());
            (hash_msg, keccak(&buf).into())
        };

        assert!(!self.affirmationsSigned(hash_sender));
        self.setAffirmationsSigned(hash_sender, true);

        let mut signed: U256 = self.numAffirmationsSigned(hash_msg);

        assert!(!self.isAlreadyProcessed(signed));

        signed += 1.into();

        self.setNumAffirmationsSigned(hash_msg, signed);

        self.SignedForAffirmation(pwasm_ethereum::sender(), transaction_hash);

        if signed >= self.0.required_signatures().into() {
            // If the bridge contract does not own enough tokens to transfer
            // it will couse funds lock on the home side of the bridge
            self.setNumAffirmationsSigned(hash_msg, self.markAsProcessed(signed));
            assert!(self.onExecuteAffirmation(recipient, value));
            self.AffirmationCompleted(recipient, value, transaction_hash);
        }
    }

    fn submitSignature(&mut self, signature: Vec<u8>, message: Vec<u8>) {
        // check that the sender is a validator
        self.0.check_validator();

        let sender = pwasm_ethereum::sender();

        // ensure that `signature` is really `message` signed by `msg.sender`
        // `parse_message::recover_address_from_signed_message` also validates
        // the signature and message and ensures both are valid.
        assert_eq!(
            sender,
            parse_message::recover_address_from_signed_message(&signature, &message),
            "Message not signed by sender"
        );
        let hash_msg: [u8; 32] = keccak(&message).into();
        let hash_sender = {
            let mut q = [0; 52];
            q[..20].copy_from_slice(sender.as_ref());
            q[20..].copy_from_slice(&hash_msg);
            keccak(&q[..])
        }
        .into();
        let mut signed = self.numMessagesSigned(hash_msg);
        assert!(!self.isAlreadyProcessed(signed));
        signed += 1.into();
        // the check above assures that the case when the value could be overflew will not happen in the addition operation below
        if signed > 1.into() {
            // Duplicated signatures
            assert!(!self.messagesSigned(hash_sender));
        } else {
            self.setMessages(hash_msg, message);
        }
        self.setMessagesSigned(hash_sender, true);

        let signIdx = {
            let mut q = [0; 64];
            q[..32].copy_from_slice(&hash_msg);
            q[32..].copy_from_slice(&<[u8; 32]>::from(signed - 1));
            keccak(&q[..]).into()
        };

        self.setSignatures(signIdx, signature);

        self.setNumMessagesSigned(hash_msg, signed);

        self.SignedForUserRequest(sender, hash_msg);

        let req_sigs = self.0.required_signatures().into();
        if signed >= req_sigs {
            self.setNumMessagesSigned(hash_msg, self.markAsProcessed(signed));
            self.CollectedSignatures(sender, hash_msg, req_sigs);
        }
    }

    fn numAffirmationsSigned(&self, _withdrawl: [u8; 32]) -> U256 {
        unimplemented!();
    }

    fn affirmationsSigned(&self, _withdrawal: [u8; 32]) -> bool {
        unimplemented!();
    }

    #[allow(unsafe_code)]
    fn signature(&self, hash: [u8; 32], index: U256) -> Vec<u8> {
        // FIXME is this unsafe approach worth it??????
        let signIdx = unsafe {
            let mut q: [u8; 64] = core::mem::uninitialized();
            core::ptr::copy_nonoverlapping(hash.as_ptr(), q.as_mut_ptr(), 32);
            index.to_little_endian(core::slice::from_raw_parts_mut(
                q.as_mut_ptr().offset(32),
                32,
            ));
            keccak(&q[..]).into()
        };
        self.signatures(signIdx)
    }

    fn messagesSigned(&self, _message: [u8; 32]) -> bool {
        unimplemented!();
    }

    fn message(&self, hash: [u8; 32]) -> Vec<u8> {
        self.messages(hash)
    }

    fn isAlreadyProcessed(&self, number: U256) -> bool {
        let mut q = [0; 32];
        number.to_little_endian(&mut q);
        (q[31] & 0x80) != 0
    }

    fn numMessagesSigned(&self, _message: [u8; 32]) -> U256 {
        unimplemented!();
    }

    fn requiredMessageLength(&self) -> U256 {
        parse_message::REQUIRED_MESSAGE_LENGTH.into()
    }
}

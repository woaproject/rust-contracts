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
mod Validateable;
mod basic_home_bridge;
mod ownable;
use super::bridge_validators::IBridgeValidators;
use core::usize;
use pwasm_abi::eth::EndpointInterface;
use pwasm_abi_derive::eth_abi;
use pwasm_ethereum::{block_number, read, sender, write};
use pwasm_std::{
    types::{Address, H256, U256},
    Vec,
};
extern crate tiny_keccak;
use tiny_keccak::Keccak;

include!(concat!(env!("OUT_DIR"), "/hashes.rs"));

#[allow(non_snake_case)]
#[eth_abi(BridgeValidators)]
trait BridgeValidatorsTrait: Ownable {
    /// Fired when a validator is added.
    #[event]
    fn ValidatorAdded(&mut self, indexed_validator: Address);

    /// Fired when a validator is removed.
    #[event]
    fn ValidatorRemoved(&mut self, indexed_validator: Address);

    /// Fired when the required number of signatures is changed.
    #[event]
    fn RequiredSignaturesChanged(&mut self, requiredSignatures: U256);

    fn initialize(
        &mut self,
        requiredSignatures: U256,
        initialValidators: Vec<Address>,
        owner: Address,
    ) -> bool;

    fn addValidator(&mut self, validator: Address);

    fn removeValidator(&mut self, validator: Address);

    fn setRequiredSignatures(&mut self, requiredSignatures: U256);

    #[constant]
    fn getBridgeValidatorsInterfacesVersion(&self) -> (u64, u64, u64);

    #[constant]
    fn requiredSignatures(&self) -> U256;

    #[constant]
    fn validatorCount(&self) -> U256;

    #[constant]
    fn validators(&self, validator: Address) -> bool;

    #[constant]
    fn isValidator(&self, validator: Address) -> bool;

    #[constant]
    fn isInitialized(&self) -> bool;

    #[constant]
    fn deployedAtBlock(&self) -> U256;

    /// Event to show ownership has been transferred
    ///
    /// <dl>
    ///  <dt>previousOwner</dt><dd>representing the address of the previous owner</dd>
    ///  <dt>newOwner</dt><dd>representing the address of the new owner</dd>
    /// </dl>
    #[allow(non_snake_case)]
    #[event]
    fn OwnershipTransferred(&mut self, previousOwner: Address, newOwner: Address);

    /// Throws if called by any account other than the owner.
    ///
    /// # Panics
    ///
    /// Panics if called by any account other than the owner.
    #[allow(non_snake_case)]
    #[constant]
    fn onlyOwner(&self);

    /// Tells the address of the owner.
    ///
    /// Returns the address of the owner.
    #[allow(non_snake_case)]
    #[constant]
    fn owner(&self) -> Address;

    /**
     * Allows the current owner to transfer control of the contract to a newOwner.
     *
     * `newOwner`: the address to transfer ownership to.
     */
    #[allow(non_snake_case)]
    fn transferOwnership(&mut self, newOwner: Address) {
        self.onlyOwner();
        assert_ne!(newOwner, Address::zero());
        self.OwnershipTransferred(self.owner(), newOwner);
        set_owner(H256::from(newOwner).into())
    }
}
struct BridgeValidatorsImpl;

impl BridgeValidatorsTrait for BridgeValidatorsImpl {
    fn initialize(
        &mut self,
        required_signatures: U256,
        initial_validators: Vec<Address>,
        owner: Address,
    ) -> bool {
        assert!(required_signatures < usize::MAX.into());
        assert!(!self.isInitialized());
        assert_ne!(owner, Address::zero());
        set_owner(H256::from(owner).into());
        assert_ne!(required_signatures, U256::zero());
        let mut buf = [0; 32];
        assert!(U256::from(initial_validators.len()) >= required_signatures);
        for &inital_validator in &initial_validators {
            assert_ne!(inital_validator, Address::zero());
            assert!(!self.isValidator(inital_validator));
            (self.validatorCount() + 1u32).to_little_endian(&mut buf);
            set_validatorCount(buf);
            self.set_validator(inital_validator, true);
            self.ValidatorAdded(inital_validator);
        }
        set_requiredSignatures(required_signatures.into());
        set_deployedAtBlock(U256::from(block_number()).into());
        let q = [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0,
        ];
        set_isInitialized(q);
        self.RequiredSignaturesChanged(required_signatures);
        return self.isInitialized();
    }

    fn addValidator(&mut self, validator: Address) {
        assert_eq!(sender(), self.owner());
        assert_ne!(validator, Address::zero());
        assert!(!self.isValidator(validator));
        set_validatorCount((U256::from(get_validatorCount()) + U256::from(1)).into());
        self.set_validator(validator, true);
        self.ValidatorAdded(validator);
    }

    fn removeValidator(&mut self, validator: Address) {
        assert_eq!(sender(), self.owner());
        let old_validator_count = self.validatorCount();
        assert!(old_validator_count > self.requiredSignatures());
        assert!(self.isValidator(validator));
        self.set_validator(validator, false);
        set_validatorCount((old_validator_count - U256::from(1)).into());
        self.ValidatorRemoved(validator);
    }

    fn setRequiredSignatures(&mut self, required_signatures: U256) {
        assert_eq!(sender(), self.owner());
        assert!(self.validatorCount() >= required_signatures);
        assert_ne!(required_signatures, 0.into());
        assert!(required_signatures <= usize::MAX.into());
        set_requiredSignatures(required_signatures.into());
        self.RequiredSignaturesChanged(required_signatures);
    }

    fn requiredSignatures(&self) -> U256 {
        U256::from(get_requiredSignatures())
    }

    fn deployedAtBlock(&self) -> U256 {
        U256::from(get_deployedAtBlock())
    }

    fn getBridgeValidatorsInterfacesVersion(&self) -> (u64, u64, u64) {
        (2, 0, 0)
    }

    fn isValidator(&self, validator: Address) -> bool {
        self.validators(validator)
    }

    fn validatorCount(&self) -> U256 {
        U256::from(get_validatorCount())
    }

    fn validators(&self, validator: Address) -> bool {
        let mut hasher = Keccak::new_sha3_256();
        hasher.update(b"validators");
        hasher.update(&validator[..]);
        let mut q = [0; 32];
        hasher.finalize(&mut q);
        read(&q.into())
            == [
                1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ]
    }

    fn isInitialized(&self) -> bool {
        get_isInitialized()[..] == <[u8; 32]>::from(U256::from(0))[..]
    }

    #[allow(non_snake_case)]
    fn onlyOwner(&self) {
        assert_eq!(sender(), self.owner())
    }

    fn owner(&self) -> Address {
        H256::from(get_owner()).into()
    }
}

#[no_mangle]
pub fn call() {
    let mut endpoint = BridgeValidators::new(BridgeValidatorsImpl);
    pwasm_ethereum::ret(&endpoint.dispatch(&pwasm_ethereum::input()));
}

#[no_mangle]
pub fn deploy() {
    let mut endpoint = BridgeValidators::new(BridgeValidatorsImpl);
    endpoint.dispatch_ctor(&pwasm_ethereum::input());
}

impl BridgeValidatorsImpl {
    fn set_validator(&self, validator: Address, is_validator: bool) {
        let mut hasher = Keccak::new_sha3_256();
        hasher.update(b"validators");
        hasher.update(&validator[..]);
        let mut q = [0; 32];
        hasher.finalize(&mut q);
        write(
            &q.into(),
            &[
                is_validator.into(),
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        )
    }
}

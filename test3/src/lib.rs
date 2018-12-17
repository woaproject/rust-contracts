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
#![no_std]
#![feature(alloc)]
extern crate alloc;

use pwasm_abi_derive::eth_abi;
use pwasm_ethereum;

use pwasm_std::{
    types::{Address, U256},
    Vec,
};
mod block_reward;
mod bridge_validators;
mod parse_message;
mod upgradeability;
mod upgradeable_contracts;
use self::upgradeability::upgradeability_storage::{create_storage_input, UpgradeabilityStorage};
// Declares the dispatch and dispatch_ctor methods
use pwasm_abi::eth::EndpointInterface;

#[eth_abi(BurnableMintableERC677Token)]
trait IBurnableMintableERC677Token: IERC677 {
    fn mint(&self, _: Address, _: U256) -> bool;
    fn burn(&self, _value: U256);
    fn claimTokens(&self, _token: Address, _to: Address);
}

#[eth_abi(ERC677)]
trait IERC677: ERC20 {
    #[event]
    fn Transfer(indexed_from: Address, indexed_to: Address, value: U256, data: Vec<u8>);
    fn transferAndCall(&self, _: Address, _: U256, _: Vec<u8>) -> bool;
}
#[cfg(none)]
#[no_mangle]
pub fn call() {
    let mut endpoint =
        UpgradeabilityStorage::new(create_storage_input(U256::zero(), Address::zero()));
    // Read http://solidity.readthedocs.io/en/develop/abi-spec.html#formal-specification-of-the-encoding for details
    pwasm_ethereum::ret(&endpoint.dispatch(&pwasm_ethereum::input()));
}

#[cfg(none)]
#[no_mangle]
pub fn deploy() {
    let mut endpoint =
        UpgradeabilityStorage::new(create_storage_input(U256::zero(), Address::zero()));
    //
    endpoint.dispatch_ctor(&pwasm_ethereum::input());
}

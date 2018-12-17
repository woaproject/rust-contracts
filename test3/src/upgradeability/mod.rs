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
#![allow(non_snake_case, dead_code)]
pub mod eternal_storage;
pub mod upgradeability_storage;

use pwasm_abi_derive::eth_abi;
use pwasm_ethereum::{address, call, call_code, gas_left, sender, value};
use pwasm_std::types::{Address, U256};
use pwasm_std::Vec;

mod internal {
    pub struct Q;
    pub trait UpgradeabilityProxy {
        fn _upgradeTo(&mut self, version: super::U256, implementation: super::Address);
        fn trivial(&self) -> Q {
            Q
        }
    }
}

/**
 * @title OwnedUpgradeabilityProxy
 * @dev This contract combines an upgradeability proxy with basic authorization control functionalities
 */
#[eth_abi(OwnedUpgradeabilityProxy)]
trait OwnedUpgradeabilityProxyTrait {
    /**
     * @dev Event to show ownership has been transferred
     * @param previousOwner representing the address of the previous owner
     * @param newOwner representing the address of the new owner
     */
    #[event]
    fn ProxyOwnershipTransferred(previousOwner: Address, newOwner: Address);

    /// The constructor sets the original owner of the contract to the sender account.
    fn constructor(&mut self);

    /// Returns the address of the proxy owner.
    #[constant]
    fn proxyOwner(&self) -> Address;

    /// Asserts that the current owner is the sender of the message.
    ///
    /// # Panics
    ///
    /// Panics if the current execution was not triggered by the owner of this contract.
    fn onlyProxyOwner(&self) {
        assert_eq!(sender(), self.proxyOwner())
    }

    /// Allows the current owner to transfer control of the contract to address `newOwner`.
    ///
    /// # Panics
    ///
    /// Panics if `msg.sender` is not the current owner.
    fn transferProxyOwnership(&mut self, newOwner: Address);

    /// Allows the upgradeability owner to upgrade the current version of the proxy.
    /// The new implementation address is set to `implementation`,
    /// and the new version is set to `version`.
    ///
    /// `implementation` must not be the zero address.  `version` must be strictly
    /// greater than the current version.
    ///
    /// # Panics
    ///
    /// Panics if any of the following are true:
    ///
    /// * `implementation` is the zero address.
    /// * `version` is &le; the old version.
    /// * `msg.sender` is not the current owner of the contract.
    fn upgradeTo(&mut self, version: U256, implementation: Address);

    /**
     * @dev Allows the upgradeability owner to upgrade the current version of the proxy and call the new implementation
     * to initialize whatever is needed through a low level call.
     * @param version representing the version name of the new implementation to be set.
     * @param implementation representing the address of the new implementation to be set.
     * @param data represents the msg.data to bet sent in the low level call. This parameter may include the function
     * signature of the implementation to be called with the needed payload
     */
    //#[payable]
    fn upgradeToAndCall(&mut self, version: U256, implementation: Address, data: Vec<u8>) {
        self.upgradeTo(version, implementation);
        call(gas_left(), &address(), value(), &data, &mut []).expect("Upgrade failed")
    }

    /**
     * @dev This event will be emitted every time the implementation gets upgraded
     * @param version representing the version name of the upgraded implementation
     * @param implementation representing the address of the upgraded implementation
     */
    #[event]
    fn Upgraded(version: U256, indexed_implementation: Address);
}

/// # UpgradeabilityOwnerStorage
///
/// This contract keeps track of the upgradeability owner
struct OwnedUpgradeabilityProxyImpl {
    /// Contract version
    version: U256,
    /// Implementation
    implementation: Address,
    /// Owner of the contract
    upgradeability_owner: Address,
}

impl OwnedUpgradeabilityProxyTrait for OwnedUpgradeabilityProxyImpl {
    fn transferProxyOwnership(&mut self, newOwner: Address) {
        self.onlyProxyOwner();
        assert_ne!(newOwner, Address::zero());
        self.ProxyOwnershipTransferred(sender(), newOwner);
        self.upgradeability_owner = newOwner;
    }

    fn proxyOwner(&self) -> Address {
        self.upgradeability_owner
    }

    fn constructor(&mut self) {
        self.upgradeability_owner = sender()
    }

    fn upgradeTo(&mut self, version: U256, implementation: Address) {
        self.onlyProxyOwner();
        assert_ne!(sender(), implementation);
        assert!(version > self.version);
        self.version = version;
        self.implementation = implementation;
        self.Upgraded(version, implementation);
    }
}

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
use pwasm_abi_derive::eth_abi;
use pwasm_std::types::{Address, U256};

/**
 * # UpgradeabilityStorage
 *
 * This contract holds all the necessary state variables to support the upgrade functionality
 */
#[eth_abi(UpgradeabilityStorage)]
trait UpgradeabilityStorageTrait {
    /**
     * Tells the version name of the current implementation.
     *
     * Returns a string representing the name of the current version.
     */
    #[constant]
    fn version(&self) -> U256;

    /**
     * @dev Tells the address of the current implementation
     * @return address of the current implementation
     */
    #[constant]
    fn implementation(&self) -> Address;
}

pub fn create_storage_input(
    version: U256,
    implementation: Address,
) -> impl UpgradeabilityStorageTrait {
    UpgradeabilityStorageImpl {
        version,
        implementation,
    }
}

struct UpgradeabilityStorageImpl {
    // Version name of the current implementation
    version: U256,

    // Address of the current implementation
    implementation: Address,
}

impl UpgradeabilityStorageTrait for UpgradeabilityStorageImpl {
    /**
     * Tells the version name of the current implementation.
     *
     * Returns a string representing the name of the current version.
     */
    fn version(&self) -> U256 {
        self.version
    }

    /**
     * @dev Tells the address of the current implementation
     * @return address of the current implementation
     */
    fn implementation(&self) -> Address {
        self.implementation
    }
}

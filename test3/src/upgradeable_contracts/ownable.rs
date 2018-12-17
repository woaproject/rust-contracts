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
use super::super::upgradeability::eternal_storage;
use pwasm_ethereum::sender;
use pwasm_abi_derive::eth_abi;
use pwasm_std::types::{Address};
/// # Ownable
///
/// This contract has an owner address providing basic authorization control
#[eth_abi(Ownable)]
trait OwnableTrait: eternal_storage::EternalStorage {
}
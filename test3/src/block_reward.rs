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
use pwasm_std::types::{Address, U256};

pub trait IBlockReward {
    fn add_extra_receiver(&mut self, amount: U256, receiver: Address);
    fn minted_totally(&self) -> U256;
    fn minted_totally_by_bridge(bridge: Address) -> U256;
    fn bridges_allowed_length(&self) -> U256;
}

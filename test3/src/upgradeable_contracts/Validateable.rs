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

use super::super::bridge_validators::IBridgeValidators;
use pwasm_ethereum::sender;
use pwasm_std::{types::U256, Box};

pub struct Validateable(Box<dyn IBridgeValidators>);

impl Validateable {
    pub fn validator_contract(&self) -> &dyn IBridgeValidators {
        &*self.0
    }

    pub fn check_validator(&self) {
        assert!(
            self.0.is_validator(sender()),
            "This method can only be called by a validator"
        )
    }

    pub fn check_owner(&self) {
        assert_eq!(
            self.0.owner(),
            sender(),
            "This method can only be called by the contract owner"
        )
    }

    pub fn required_signatures(&self) -> usize {
        self.0.required_signatures()
    }
}

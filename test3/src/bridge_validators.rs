use pwasm_std::types::{Address, U256};

pub trait IBridgeValidators {
    fn is_validator(&self, validator: Address) -> bool;
    fn required_signatures(&self) -> usize;
    fn owner(&self) -> Address;
}
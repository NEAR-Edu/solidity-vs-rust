use near_sdk::{
    borsh::{self, *},
    collections::*,
    json_types::*,
    serde::{self, *},
    *,
};

mod contract;
pub use contract::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}

use near_sdk::{
    borsh::{self, *},
    *,
};

mod contract;
pub use contract::*;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}

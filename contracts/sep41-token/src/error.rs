use soroban_sdk::contracterror;

#[contracterror]
#[derive(Debug, Copy, PartialEq, Eq, Clone)]
pub enum ContractError {
    InsufficientFunds = 1,
    InsufficientAllowance = 2,
    Unauthorized = 3,
}
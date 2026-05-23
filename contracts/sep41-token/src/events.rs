use soroban_sdk::{contractevent, Address};

#[contractevent]
#[derive(Debug, PartialEq, Eq, Clone)]

pub struct Transfer {
    #[topic]
    pub from: Address,
    #[topic]
    pub to: Address,
    pub amount: i128,
}

#[contractevent]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Approval {
    #[topic]
    pub from: Address,
    #[topic]
    pub spender: Address,
    pub amount: i128,
    pub live_until_ledger: u32,
}

#[contractevent]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Burn {
    #[topic]
    pub from: Address,
    pub amount: i128,
}

#[contractevent]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Mint {
    #[topic]
    pub admin: Address,

    #[topic]
    pub to: Address,
    pub amount: i128,
}

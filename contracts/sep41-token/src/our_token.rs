use soroban_sdk::{contract, contractimpl, Address, Env, IntoVal, String};

use crate::{
    error::ContractError,
    events::{Approval, Transfer, Burn, Mint},
    storage::{AllowanceKey, DataKey},
    token_trait::TokenInterface,
};

#[contract]
pub struct SibToken;

#[contractimpl]
impl SibToken {

    pub fn __constructor(env: Env, admin: Address, initial_supply: i128) {
        // admin.require_auth();

        env.storage().persistent().set(&DataKey::Admin, &admin);

        env.storage().persistent().set(&DataKey::Balance(admin.clone()), &initial_supply);

        env.storage().persistent().set(&DataKey::TotalSupply, &initial_supply);
    }
}

#[contractimpl]
impl TokenInterface for SibToken {
     fn balance(env: Env, id: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Balance(id))
            .unwrap_or(0)
    }

     fn allowance(env: Env, from: Address, spender: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Allowance(AllowanceKey { from, spender }))
            .unwrap_or(0)
    }

     fn approve(
        env: Env,
        from: Address,
        spender: Address,
        amount: i128,
        live_until_ledger: u32,
    ) -> Result<(), ContractError> {
        from.require_auth();

        let from_balance = Self::balance(env.clone(), from.clone());

        if from_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        let key = DataKey::Allowance(AllowanceKey {
            from: from.clone(),
            spender: spender.clone(),
        });

        env.storage().persistent().set(&key, &amount);

        Approval {
            from,
            spender,
            amount: amount.try_into().unwrap(),
            live_until_ledger: live_until_ledger.into_val(&env),
        }
        .publish(&env);

        Ok(())
    }

     fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128,
    ) -> Result<(), ContractError> {
        from.require_auth();
        let sender_balance = Self::balance(env.clone(), from.clone());

        let receiver_balance = Self::balance(env.clone(), to.clone());

        if sender_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        let receiver_balance = Self::balance(env.clone(), to.clone());

        // env.storage()
        //     .persistent()
        //     .set(&sender_balance, &(sender_balance - amount));

        // env.storage()
        //     .persistent()
        //     .set(&receiver_balance, &(receiver_balance + amount));

        env.storage()
            .persistent()
            .set(&DataKey::Balance(from.clone()), &(sender_balance - amount));

        env.storage()
            .persistent()
            .set(&DataKey::Balance(to.clone()), &(receiver_balance + amount));

        Transfer {
            from,
            to,
            amount: amount.try_into().unwrap(),
        }
        .publish(&env);

        Ok(())
    }

    // Add the transfer_from function here ------ Assign 1
     fn transfer_from(env: Env,spender: Address, from: Address, to: Address, amount: i128) -> Result<(), ContractError> {
        spender.require_auth();

        let allowance = Self::allowance(env.clone(), from.clone(), spender.clone());

        if allowance < amount {
            return Err(ContractError::InsufficientAllowance);
        }

        let from_balance = Self::balance(env.clone(), from.clone());

        if from_balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        let to_balance = Self::balance(env.clone(), to.clone());

        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(from_balance - amount));

        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(to_balance + amount));

        env.storage().persistent().set(
            &DataKey::Allowance(AllowanceKey { from: from.clone(), spender: spender.clone() }),
            &(allowance - amount),
        );
        
        // env.storage().persistent().set(&key, &(allowance - amount));

        Transfer{
            from,
            to,
            amount,
        }

       .publish(&env);

       Ok(())

    }

    // burn function ----------Assign 2

     fn burn(env: Env, from: Address, amount: i128) -> Result<(), ContractError> {
        from.require_auth();

        let balance = Self::balance(env.clone(), from.clone());

        if balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        let total_supply: i128 = env.storage().persistent().get(&DataKey::TotalSupply).unwrap_or(0);

        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(balance - amount));

        env.storage().persistent().set(&DataKey::TotalSupply, &(total_supply - amount));

        Burn {
            from,
            amount,
        }
        .publish(&env);

        Ok(())

    }
    
    // burn_from function ----------- Assign 3
     fn burn_from (env: Env, spender: Address, from:Address, amount: i128) -> Result<(), ContractError> {
        spender.require_auth();

        let allowance = Self::allowance(env.clone(), from.clone(), spender.clone());

        if allowance < amount {
            return Err(ContractError::InsufficientAllowance);
        }


        let balance = Self::balance(env.clone(), from.clone());

        if balance < amount {
            return Err(ContractError::InsufficientFunds);
        }

        let total_supply: i128 = env.storage().persistent().get(&DataKey::TotalSupply).unwrap_or(0);

        env.storage().persistent().set(&DataKey::Balance(from.clone()), &(balance -amount));

        env.storage().persistent().set(&DataKey::TotalSupply, &(total_supply - amount));

        // let key = DataKey::Allowance(AllowanceKey{
        //     from: from.clone(),
        //     spender: spender.clone(),
         
        // });

        // env.storage().persistent().set(&key, &(allowance - amount));

        env.storage().persistent().set(
            &DataKey::Allowance(AllowanceKey {
                from: from.clone(),
                spender: spender.clone(),
            }),
            &(allowance - amount),
        );

        Burn {
            from,
            amount,
        }
        .publish(&env);

        Ok(())

    }

    // Mint function ------- Assign 4

     fn mint (env: Env, to: Address, amount: i128 ) -> Result<(), ContractError> {

        let admin: Address = env.storage().persistent().get(&DataKey::Admin).unwrap();

        admin.require_auth();

        let balance = Self::balance(env.clone(), to.clone());

        let total_supply = env.storage().persistent().get(&DataKey::TotalSupply).unwrap_or(0);

        env.storage().persistent().set(&DataKey::Balance(to.clone()), &(balance + amount));

        env.storage().persistent().set(&DataKey::TotalSupply, &(total_supply + amount));

        Mint {
            admin,
            to, 
            amount,
        }
        .publish(&env);
        
        Ok(())

    }

     fn decimals(_env: Env) -> u32 {
        18
    }

     fn name(env: Env) -> String {
        String::from_str(&env, "SibToken")
    }

     fn symbol(env: Env) -> String {
        String::from_str(&env, "SIB")
    }


}
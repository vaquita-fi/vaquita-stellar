#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, String, Vec, Symbol, token
};
use blend_contract_sdk::pool::{Request, Client};

// ==================== DATA STRUCTS ====================

#[derive(Clone)]
#[contracttype]
pub struct Position {
    owner: Address,
    amount: i128,
    shares: i128,
    finalization_time: u64,
    lock_period: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct Period {
    reward_pool: i128,
    total_shares: i128,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Owner,
    Token,
    PoolAddress,
    BasisPoints,
    EarlyWithdrawalFee,
    ProtocolFees,
    Positions(String),
    Periods(u64),
    SupportedLockPeriod(u64),
}

// ==================== CONTRACT ====================

#[contract]
pub struct VaquitaPool;

#[contractimpl]
impl VaquitaPool {
    // ---------- Initialization ----------
    pub fn initialize(env: Env, owner: Address, token: Address, pool_address: Address, lock_periods: Vec<u64>) {
        env.storage().instance().set(&DataKey::Owner, &owner);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::PoolAddress, &pool_address);
        env.storage().instance().set(&DataKey::BasisPoints, &10000i128);
        env.storage().instance().set(&DataKey::EarlyWithdrawalFee, &0i128);
        env.storage().instance().set(&DataKey::ProtocolFees, &0i128);

        for lp in lock_periods.iter() {
            env.storage().instance().set(&DataKey::SupportedLockPeriod(lp.clone()), &true);
        }
    }

    // ---------- Owner Check ----------
    fn require_owner(env: &Env, caller: Address) {
        let owner: Address = env.storage().instance().get(&DataKey::Owner).unwrap();
        if caller != owner {
            panic!("Not owner");
        }
    }

    // ---------- Deposit ----------
    pub fn deposit(env: Env, caller: Address, deposit_id: String, amount: i128, period: u64) {
        caller.require_auth();
    
        if amount <= 0 {
            panic!("Invalid amount");
        }
        if env.storage().instance().has(&DataKey::Positions(deposit_id.clone())) {
            panic!("Deposit already exists");
        }
        let supported: bool = env.storage().instance()
            .get(&DataKey::SupportedLockPeriod(period))
            .unwrap_or(false);
        if !supported {
            panic!("Invalid period");
        }
    
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let pool_address: Address = env.storage().instance().get(&DataKey::PoolAddress).unwrap();
        let contract_address = env.current_contract_address();
        let current_ledger = env.ledger().sequence();
        let finalization_time = env.ledger().timestamp() + period;
    
        // Step 1: Pull tokens from user
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&caller, &contract_address, &amount);
    
        // Step 2: Approve pool to spend from contract
        token_client.approve(
            &contract_address,
            &pool_address,
            &amount,
            &(current_ledger + 600),
        );
    
        // Step 3: Track shares (simplified)
        let shares = amount;
    
        let position = Position {
            owner: caller.clone(),
            amount,
            shares,
            finalization_time,
            lock_period: period,
        };
        env.storage().instance().set(&DataKey::Positions(deposit_id.clone()), &position);

        // Step 5: Supply to Blend on contract’s behalf
        let request = Request { 
            request_type: 0u32, // Supply
            address: token.clone(),
            amount,
        };
        let requests = Vec::from_array(&env, [request]);
        let pool_client = Client::new(&env, &pool_address);
        pool_client.submit_with_allowance(&contract_address, &contract_address, &contract_address, &requests);
    
        // Step 6: Update total shares for this period
        let mut period_data: Period = env.storage().instance()
            .get(&DataKey::Periods(period))
            .unwrap_or(Period { reward_pool: 0, total_shares: 0 });
        period_data.total_shares += shares;
        env.storage().instance().set(&DataKey::Periods(period), &period_data);
    
        // Step 7: Emit event
        env.events().publish(
            (Symbol::new(&env, "deposit"), caller),
            (deposit_id, token, amount, shares),
        );
    }

    // ---------- Withdraw ----------
    pub fn withdraw(env: Env, caller: Address, deposit_id: String) {
        caller.require_auth();
        
        let position: Position = env.storage().instance().get(&DataKey::Positions(deposit_id.clone()))
            .unwrap_or_else(|| panic!("Position not found"));

        if caller != position.owner {
            panic!("Not position owner");
        }

        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let pool_address: Address = env.storage().instance().get(&DataKey::PoolAddress).unwrap();
        let contract_address = env.current_contract_address();

        // Step 1: Withdraw from Blend - Blend will automatically calculate and return the interest
        let request = Request { 
            request_type: 1u32, // Withdraw
            address: token.clone(),
            amount: position.amount,
        };
        let requests = Vec::from_array(&env, [request]);
        let pool_client = Client::new(&env, &pool_address);
        pool_client.submit(&contract_address, &contract_address, &contract_address, &requests);

        let now = env.ledger().timestamp();
        let mut amount_to_transfer = position.amount;
        let mut reward: i128 = 0;

        let mut period_data: Period = env.storage().instance().get(&DataKey::Periods(position.lock_period)).unwrap();

        if now < position.finalization_time {
            // Early withdrawal fee
            let interest = if position.amount > position.amount { position.amount - position.amount } else { 0 };
            let early_fee: i128 = env.storage().instance().get(&DataKey::EarlyWithdrawalFee).unwrap();
            let fee_amount = (interest * early_fee) / 10000;
            let remaining_interest = interest - fee_amount;
            let mut protocol_fees: i128 = env.storage().instance().get(&DataKey::ProtocolFees).unwrap();
            protocol_fees += fee_amount;
            env.storage().instance().set(&DataKey::ProtocolFees, &protocol_fees);
            period_data.reward_pool += remaining_interest;
            amount_to_transfer -= interest;
        } else {
            // Late withdrawal with rewards
            reward = Self::calculate_reward(&period_data, position.shares);
            period_data.reward_pool -= reward;
            amount_to_transfer += reward;
        }

        // Step 3: Transfer final amount from contract back to user
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&contract_address, &caller, &amount_to_transfer);

        // Update shares
        period_data.total_shares -= position.shares;
        env.storage().instance().set(&DataKey::Periods(position.lock_period), &period_data);

        // Remove position
        env.storage().instance().remove(&DataKey::Positions(deposit_id.clone()));

        // Emit event
        env.events().publish(
            (Symbol::new(&env, "withdraw"), caller.clone()),
            (deposit_id, token, amount_to_transfer, reward),
        );
    }

    fn calculate_reward(period_data: &Period, shares: i128) -> i128 {
        if period_data.total_shares == 0 {
            return 0;
        }
        (period_data.reward_pool * shares) / period_data.total_shares
    }

    // ---------- Owner functions ----------
    pub fn withdraw_protocol_fees(env: Env, caller: Address) {
        Self::require_owner(&env, caller.clone());
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let contract_address = env.current_contract_address();
        let protocol_fees: i128 = env.storage().instance().get(&DataKey::ProtocolFees).unwrap();
        
        if protocol_fees > 0 {
            let token_client = token::Client::new(&env, &token);
            token_client.transfer(&contract_address, &caller, &protocol_fees);
            env.storage().instance().set(&DataKey::ProtocolFees, &0i128);
        }
    }

    pub fn add_rewards(env: Env, caller: Address, period: u64, reward_amount: i128) {
        Self::require_owner(&env, caller.clone());
        
        // First transfer the reward tokens from owner to contract
        let token: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let contract_address = env.current_contract_address();
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&caller, &contract_address, &reward_amount);
        
        let supported: bool = env.storage().instance().get(&DataKey::SupportedLockPeriod(period)).unwrap_or(false);
        if !supported {
            panic!("Invalid period");
        }
        let mut period_data: Period = env.storage().instance().get(&DataKey::Periods(period)).unwrap_or(Period {
            reward_pool: 0,
            total_shares: 0,
        });
        period_data.reward_pool += reward_amount;
        env.storage().instance().set(&DataKey::Periods(period), &period_data);
    }

    pub fn update_early_withdrawal_fee(env: Env, caller: Address, new_fee: i128) {
        Self::require_owner(&env, caller);
        let basis_points: i128 = env.storage().instance().get(&DataKey::BasisPoints).unwrap();
        if new_fee > basis_points {
            panic!("Invalid fee");
        }
        env.storage().instance().set(&DataKey::EarlyWithdrawalFee, &new_fee);
    }

    pub fn add_lock_period(env: Env, caller: Address, new_lock_period: u64) {
        Self::require_owner(&env, caller);
        let exists: bool = env.storage().instance().get(&DataKey::SupportedLockPeriod(new_lock_period)).unwrap_or(false);
        if exists {
            panic!("Lock period already supported");
        }
        env.storage().instance().set(&DataKey::SupportedLockPeriod(new_lock_period), &true);
    }

    // ---------- View functions ----------
    pub fn get_position(env: Env, deposit_id: String) -> Option<Position> {
        env.storage().instance().get(&DataKey::Positions(deposit_id))
    }

    pub fn get_period_data(env: Env, period: u64) -> Option<Period> {
        env.storage().instance().get(&DataKey::Periods(period))
    }
}
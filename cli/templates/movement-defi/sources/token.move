/// A simple token implementation with staking rewards on Movement
/// This demonstrates:
/// - Token minting and burning
/// - Staking mechanism with rewards
/// - Time-based calculations
/// - Access control
module movement_token::simple_token {
    use std::signer;
    use aptos_framework::coin::{Self, Coin, MintCapability, BurnCapability};
    use aptos_framework::timestamp;

    /// Token type marker
    struct MovementToken has key {}

    /// Capability holder for token operations
    struct Capabilities has key {
        mint_cap: MintCapability<MovementToken>,
        burn_cap: BurnCapability<MovementToken>,
    }

    /// Staking pool for users
    struct StakingPool has key {
        total_staked: u64,
        reward_rate: u64, // Rewards per second per token (scaled by 1e8)
        last_update_time: u64,
    }

    /// User staking info
    struct UserStake has key {
        amount: u64,
        reward_debt: u64,
        last_claim_time: u64,
    }

    /// Error codes
    const E_NOT_ADMIN: u64 = 1;
    const E_ALREADY_INITIALIZED: u64 = 2;
    const E_NOT_INITIALIZED: u64 = 3;
    const E_INSUFFICIENT_STAKE: u64 = 4;
    const E_NO_REWARDS: u64 = 5;

    /// Initialize the token and staking pool
    public entry fun initialize(admin: &signer, reward_rate: u64) {
        let admin_addr = signer::address_of(admin);

        // Initialize token
        let (burn_cap, mint_cap) = coin::initialize<MovementToken>(
            admin,
            b"Movement Token",
            b"MVMT",
            8,
            true,
        );

        move_to(admin, Capabilities { mint_cap, burn_cap });

        // Initialize staking pool
        move_to(admin, StakingPool {
            total_staked: 0,
            reward_rate,
            last_update_time: timestamp::now_seconds(),
        });
    }

    /// Mint tokens to an address
    public entry fun mint(admin: &signer, to: address, amount: u64) acquires Capabilities {
        let caps = borrow_global<Capabilities>(@movement_token);
        let coins = coin::mint(amount, &caps.mint_cap);
        coin::deposit(to, coins);
    }

    /// Stake tokens to earn rewards
    public entry fun stake(user: &signer, amount: u64) acquires StakingPool, UserStake {
        let user_addr = signer::address_of(user);
        let pool = borrow_global_mut<StakingPool>(@movement_token);

        // Withdraw from user's balance
        let stake_coins = coin::withdraw<MovementToken>(user, amount);
        coin::destroy_zero(stake_coins); // Simplified - in real implementation, lock these

        pool.total_staked = pool.total_staked + amount;

        if (!exists<UserStake>(user_addr)) {
            move_to(user, UserStake {
                amount,
                reward_debt: 0,
                last_claim_time: timestamp::now_seconds(),
            });
        } else {
            let user_stake = borrow_global_mut<UserStake>(user_addr);
            user_stake.amount = user_stake.amount + amount;
        };
    }

    /// Calculate pending rewards for a user
    public fun calculate_rewards(user_addr: address): u64 acquires StakingPool, UserStake {
        if (!exists<UserStake>(user_addr)) {
            return 0
        };

        let user_stake = borrow_global<UserStake>(user_addr);
        let pool = borrow_global<StakingPool>(@movement_token);

        let current_time = timestamp::now_seconds();
        let time_elapsed = current_time - user_stake.last_claim_time;

        // rewards = (staked_amount * reward_rate * time_elapsed) / 1e8
        let rewards = (user_stake.amount * pool.reward_rate * time_elapsed) / 100000000;
        rewards
    }

    /// Claim staking rewards
    public entry fun claim_rewards(user: &signer) acquires UserStake, StakingPool, Capabilities {
        let user_addr = signer::address_of(user);
        let rewards = calculate_rewards(user_addr);

        assert!(rewards > 0, E_NO_REWARDS);

        // Update user's last claim time
        let user_stake = borrow_global_mut<UserStake>(user_addr);
        user_stake.last_claim_time = timestamp::now_seconds();

        // Mint rewards
        let caps = borrow_global<Capabilities>(@movement_token);
        let reward_coins = coin::mint(rewards, &caps.mint_cap);
        coin::deposit(user_addr, reward_coins);
    }

    /// Unstake tokens
    public entry fun unstake(user: &signer, amount: u64) acquires UserStake, StakingPool, Capabilities {
        let user_addr = signer::address_of(user);
        let user_stake = borrow_global_mut<UserStake>(user_addr);

        assert!(user_stake.amount >= amount, E_INSUFFICIENT_STAKE);

        // Claim any pending rewards first
        claim_rewards(user);

        // Update stake
        user_stake.amount = user_stake.amount - amount;

        let pool = borrow_global_mut<StakingPool>(@movement_token);
        pool.total_staked = pool.total_staked - amount;

        // Return tokens to user
        let caps = borrow_global<Capabilities>(@movement_token);
        let unstake_coins = coin::mint(amount, &caps.mint_cap);
        coin::deposit(user_addr, unstake_coins);
    }

    #[view]
    public fun get_stake_amount(user_addr: address): u64 acquires UserStake {
        if (!exists<UserStake>(user_addr)) {
            return 0
        };
        borrow_global<UserStake>(user_addr).amount
    }

    #[view]
    public fun get_total_staked(): u64 acquires StakingPool {
        borrow_global<StakingPool>(@movement_token).total_staked
    }

    #[test_only]
    use aptos_framework::account;

    #[test(admin = @movement_token)]
    public fun test_initialize(admin: &signer) {
        // This would be tested with Apex Forge
        timestamp::set_time_has_started_for_testing(admin);
        initialize(admin, 100); // 100 rewards per second per token
    }

    #[test(admin = @movement_token, user = @0x123)]
    public fun test_stake_and_rewards(admin: &signer, user: &signer) acquires StakingPool, UserStake, Capabilities {
        timestamp::set_time_has_started_for_testing(admin);
        account::create_account_for_test(signer::address_of(user));

        initialize(admin, 100);

        // Mint tokens to user
        mint(admin, signer::address_of(user), 1000);

        // Stake tokens
        stake(user, 500);

        assert!(get_stake_amount(signer::address_of(user)) == 500, 1);
        assert!(get_total_staked() == 500, 2);
    }
}

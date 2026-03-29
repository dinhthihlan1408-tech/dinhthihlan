#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Symbol,
};
use soroban_sdk::token::{Client as TokenClient};

const KEY_CONFIG: Symbol = symbol_short!("CONFIG");

#[contracttype]
#[derive(Clone)]
pub struct Config {
    pub admin: Address,
    pub usdc_token: Address,     // token used for payment (USDC)
    pub content_token: Address,  // token representing access right
    pub price: i128,             // price in USDC's smallest unit
    pub enabled: bool,
}

// Optional: simple per-(student,creator) unlock record to prevent duplicate mint.
// You can remove this if you want multiple purchases allowed.
#[contracttype]
#[derive(Clone)]
pub struct UnlockKey {
    pub student: Address,
    pub creator: Address,
}

fn read_config(e: &Env) -> Config {
    e.storage().instance().get(&KEY_CONFIG).unwrap()
}

fn write_config(e: &Env, cfg: &Config) {
    e.storage().instance().set(&KEY_CONFIG, cfg);
}

#[contract]
pub struct StellarLibrary;

#[contractimpl]
impl StellarLibrary {
    /// Initialize contract.
    /// - admin: contract owner
    /// - usdc_token: token contract address for USDC
    /// - content_token: token contract address for access token
    /// - price: e.g. 0.5 USDC in smallest units (depends on decimals)
    pub fn init(e: Env, admin: Address, usdc_token: Address, content_token: Address, price: i128) {
        // simple "only once" guard
        if e.storage().instance().has(&KEY_CONFIG) {
            panic!("already initialized");
        }

        admin.require_auth();

        let cfg = Config {
            admin,
            usdc_token,
            content_token,
            price,
            enabled: true,
        };
        write_config(&e, &cfg);
    }

    /// Admin can update config.
    pub fn set_enabled(e: Env, enabled: bool) {
        let mut cfg = read_config(&e);
        cfg.admin.require_auth();
        cfg.enabled = enabled;
        write_config(&e, &cfg);
    }

    pub fn set_price(e: Env, price: i128) {
        let mut cfg = read_config(&e);
        cfg.admin.require_auth();
        cfg.price = price;
        write_config(&e, &cfg);
    }

    /// Student purchases access:
    /// - Transfers USDC from student -> creator
    /// - Mints 1 content token to student
    ///
    /// REQUIREMENTS:
    /// - student must authorize this call
    /// - student must have approved this contract to spend `price` USDC:
    ///     usdc.approve(student, contract_address, price, expiration_ledger)
    /// - content_token must allow this contract to mint (token admin = contract)
    pub fn unlock(e: Env, student: Address, creator: Address) {
        let cfg = read_config(&e);
        if !cfg.enabled {
            panic!("disabled");
        }

        // Student must sign
        student.require_auth();

        // OPTIONAL: prevent double mint for same creator/student pair
        let key = UnlockKey {
            student: student.clone(),
            creator: creator.clone(),
        };
        if e.storage().persistent().has(&key) {
            panic!("already unlocked");
        }

        // 1) Transfer USDC payment
        let usdc = TokenClient::new(&e, &cfg.usdc_token);
        usdc.transfer(&student, &creator, &cfg.price);

        // 2) Mint 1 content token (access right)
        let content = TokenClient::new(&e, &cfg.content_token);
        content.transfer(&e.current_contract_address(), &student, &1i128);

        // Record unlock
        e.storage().persistent().set(&key, &true);
        // Optional TTL (so storage doesn't grow forever)
        // e.storage().persistent().extend_ttl(&key, 100_000, 200_000);
    }

    /// View config (for front-end)
    pub fn get_config(e: Env) -> Config {
        read_config(&e)
    }

    /// Check if student has unlocked for a creator (if you keep the record)
    pub fn is_unlocked(e: Env, student: Address, creator: Address) -> bool {
        let key = UnlockKey { student, creator };
        e.storage().persistent().has(&key)
    }
}
stellar contract invoke \
--id CAVQQD7L3Q7XNFC3WYAC4GNKGHG2V22YZ4MPMSP73FY2CMETONCMV3XA \
--source student \
--network testnet \
-- get_config \
--admin student \
--usdc_token CAVQQD7L3Q7XNFC3WYAC4GNKGHG2V22YZ4MPMSP73FY2CMETONCMV3XA \
--content_token CAVQQD7L3Q7XNFC3WYAC4GNKGHG2V22YZ4MPMSP73FY2CMETONCMV3XA \
--price 500000
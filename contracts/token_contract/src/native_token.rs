use crate::storage_types::{TokenDataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
use soroban_sdk::{Address, Env};

pub fn write_recipient_address(e: &Env, recipient: &Address) {
    let key = TokenDataKey::Recipient;

    e.storage().persistent().set(&key, recipient);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn write_token_address(e: &Env, token: &Address) {
    let key = TokenDataKey::Token;

    e.storage().persistent().set(&key, token);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn write_spot_price_per_gram(e: &Env, price: &i128) {
    let key = TokenDataKey::SpotPricePerGram;

    e.storage().persistent().set(&key, price);
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

pub fn read_recipient_address(e: &Env) -> Address {
    let key = TokenDataKey::Recipient;
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
    e.storage().persistent().get(&key).unwrap()
}

pub fn read_token_address(e: &Env) -> Address {
    let key = TokenDataKey::Token;
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
    e.storage().persistent().get(&key).unwrap()
}

pub fn read_spot_price_per_gram(e: &Env) -> i128 {
    let key = TokenDataKey::SpotPricePerGram;
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
    e.storage().persistent().get(&key).unwrap()
}

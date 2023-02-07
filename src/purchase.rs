use once_cell::sync::Lazy;
use std::{
    collections::HashSet
};

// goerli
pub static WHITELIST: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let mut m = HashSet::new();
    m.insert("0xd0a830278773282bbf635fd8e47b2447f1e9fe86");
    m
});

// get all ongoing auctions
// compute current price for each
// convert papr price -> underlying price using uniswap 
// (probably just use tick and later check slippage inclusive price?)
// try to find bids above that price 
// swap -> on callback, purchase NFT and sell -> send funds owed back to Uniswap

#![cfg_attr(RUSTC_WITH_SPECIALIZATION, feature(min_specialization))]
#![allow(clippy::integer_arithmetic)]

pub mod authorized_voters;
pub mod vote_error;
pub mod vote_instruction;
#[cfg(not(target_arch = "wasm32"))]
pub mod vote_processor;
pub mod vote_state;
pub mod vote_transaction;

#[macro_use]
extern crate solana_frozen_abi_macro;

pub use solana_sdk::vote::program::{check_id, id};

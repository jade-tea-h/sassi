#![warn(missing_docs, rustdoc::missing_doc_code_examples)]
#![allow(incomplete_features)]
#![feature(generic_const_exprs, const_trait_impl)]

//! A library for creating Swarm Intelligence AI Networks
//!
//! Pull requests are welcome!
//!
//! # Example
//!
#[doc = include_str!("../examples/basic.rs")]

pub use agent::Agent;
pub use manager::SwarmManager;

mod agent;
mod manager;

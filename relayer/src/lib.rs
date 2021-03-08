#![forbid(unsafe_code)]
#![deny(
    // warnings,
    // missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    rust_2018_idioms
)]
#![allow(clippy::large_enum_variant)]

//! IBC Relayer implementation

pub mod auth;
pub mod chain;
pub mod channel;
pub mod config;
pub mod connection;
pub mod error;
pub mod event;
pub mod foreign_client;
pub mod keyring;
pub mod keys;
pub mod light_client;
pub mod link;
pub mod macros;
pub mod relay;
pub mod transfer;
pub mod util;

//! # mms
//!
//! The `mms` package implements the Manufacturing Message Specification (MMS)
//! as defined by ISO 9506-1:2003 and ISO 9506-2:2003.
//!

#![allow(dead_code)]

mod acse;
pub mod client;
pub mod connection;
mod cotp;
pub mod itot;
mod presentation;
pub mod result;
pub mod server;
mod session;

pub use result::{Error, Result};

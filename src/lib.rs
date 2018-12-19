//! # libvplan - *indiware* timetable changes client and parser
//!
//! A client and parser for *indiware* timetable changes.
//!

#[cfg(feature = "client")]
/// The client to fetch a vplan.
pub mod client;
mod document;
/// Errors...
pub mod error;
/// Parser for dates.
pub mod parser;
/// The document.
pub mod vplan;

#[cfg(feature = "client")]
pub use crate::client::Client;
pub use crate::vplan::*;

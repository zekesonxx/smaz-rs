#![allow(unknown_lints)] // clippy
#![warn(missing_docs)]
//! smaz: a small string compression library
//!
//! Smaz is designed to compress very, very small strings of English speech.
//! ~500 bytes is around the point when gzip starts absolutely decimating smaz in performance
//!

#![feature(test)]
extern crate test;


#[macro_use] extern crate lazy_static;

#[cfg(test)]
extern crate rand;
#[cfg(feature = "cbinding")]
extern crate libc;


mod port;
#[cfg(feature = "cbinding")]
pub mod cbinding;

pub use port::*;

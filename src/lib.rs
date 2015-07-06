//! # rust_vk
//!
//! Wrapper around VK.com API for Rust.
//!
//! Provide few abstractions to easily handle process working with the API.
extern crate rustc_serialize;

mod fake_browser;
mod execute;
pub mod api;
pub mod user;
pub mod server;
pub mod app;


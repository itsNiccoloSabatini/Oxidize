#![warn(clippy::all, rust_2018_idioms)]

// Load I18n macro, for allow you use `t!` macro in anywhere.
#[macro_use]
extern crate rust_i18n;

// Init translations for current crate.
// This will load Configuration using the `[package.metadata.i18n]` section in `Cargo.toml` if exists.
// Or you can pass arguments by `i18n!` to override it.
// Config fallback missing translations to "en" locale.
// Use `fallback` option to set fallback locale.
//
i18n!("locales", fallback = "en");

mod app;
mod lang;
pub use app::OxidizeApp;
pub use lang::Language;

#![feature(iter_array_chunks)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

pub mod article;
pub mod cache;
pub mod chrome_driver;
pub mod crawl;
pub mod ditto;
pub mod favicon;
pub mod git;
pub mod github;
pub mod gravatar;
pub mod layout;
pub mod locale;
pub mod markdown;
pub mod output;
pub mod serve;
pub mod srcset;
pub mod style;
mod syntax_highlighting;
pub mod tokiort;
pub mod view;
pub mod xml;
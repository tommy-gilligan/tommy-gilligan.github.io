#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
// disable unreliable nursery lint
#![allow(clippy::missing_const_for_fn)]

pub mod article;
pub mod browser;
pub mod cache;
pub mod config;
pub mod crawl;
pub mod feed;
pub mod frontmatter;
pub mod git;
pub mod index;
pub mod layout;
pub mod locale;
pub mod markdown;
pub mod output;
pub mod prettier;
pub mod serve;
pub mod sitemap;
pub mod style;
mod syntax_highlighting;
pub mod tokiort;
pub mod view;
pub mod xml;

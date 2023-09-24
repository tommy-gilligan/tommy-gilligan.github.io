#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
// disable unreliable nursery lint
#![allow(clippy::missing_const_for_fn)]

pub const TITLE: &str = "Tommy's Blog";
pub const SCREENSHOTS: &str = "screenshots";
pub const ARTICLES: &str = "articles";
pub const ASSETS: &str = "assets";
pub const SITE: &str = "_site";

pub mod article;
pub mod asset;
pub mod browser;
pub mod cache;
pub mod crawl;
pub mod frontmatter;
pub mod git;
pub mod layout;
pub mod locale;
pub mod markdown;
pub mod output;
pub mod prettier;
pub mod serve;
pub mod shell;
mod syntax_highlighting;
pub mod terminal;
pub mod view;
pub mod watch;
pub mod xml;

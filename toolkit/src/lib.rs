#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]
// disable unreliable nursery lint
#![allow(clippy::missing_const_for_fn)]

pub const TITLE: &str = "Tommy's Blog";
pub const SCREENSHOTS: &str = "screenshots";
pub const ARTICLES: &str = "articles";
pub const SITEMAP: &str = "sitemap.xml";
pub const SITE: &str = "_site";
pub const STYLE: &str = "style.css";

pub mod article;
pub mod browser;
pub mod cache;
pub mod crawl;
pub mod css;
pub mod frontmatter;
pub mod git;
pub mod index;
pub mod layout;
pub mod locale;
pub mod markdown;
pub mod output;
pub mod prettier;
pub mod rss;
pub mod serve;
pub mod shell;
pub mod sitemap;
mod syntax_highlighting;
pub mod tokiort;
pub mod view;
pub mod xml;

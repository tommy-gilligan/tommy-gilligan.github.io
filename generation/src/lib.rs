#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

mod author;
pub mod chrome_driver;
pub mod crawl;
pub mod generate;
pub mod history;
pub mod layout;
mod link_list;
pub mod output;
pub mod page;
pub mod cache;
pub mod style;
mod syntax_highlighting;

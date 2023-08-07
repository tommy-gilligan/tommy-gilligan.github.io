#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

mod author;
pub mod chrome_driver;
mod config;
pub mod generate;
mod history;
mod layout;
mod link_list;
pub mod page;
pub mod render;
mod style;
mod syntax_highlighting;

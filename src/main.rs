#![feature(once_cell)]
#![feature(entry_insert)]
#![feature(try_find)]

use color_eyre::Result;

pub mod models;
pub mod network;
pub mod repository;
pub mod utils;
pub mod terminal;


fn main() -> Result<()> {
    color_eyre::install()?;
    println!("Hello, world!");

    Ok(())
}

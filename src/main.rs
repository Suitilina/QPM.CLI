#![feature(once_cell)]
#![feature(entry_insert)]
#![feature(try_find)]
#![feature(iterator_try_collect)]
#![feature(let_chains)]
use color_eyre::Result;

pub mod models;
pub mod network;
pub mod repository;
pub mod utils;
pub mod terminal;
pub mod resolver;
pub mod commands;

#[cfg(benchmark)]
mod benchmark;

#[cfg(test)]
mod tests;


fn main() -> Result<()> {
    color_eyre::install()?;
    println!("Hello, world!");

    Ok(())
}

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Import the macro.
#[macro_use]
extern crate error_chain;

pub use errors::*;
use std::path::Path;

mod errors;
mod fs_tree;
mod named_tree;

struct Config {
    root_dir: Box<Path>,
    block_size: i32,
}

quick_main!(|| -> Result<()> { Ok(()) });

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Import the macro.
#[macro_use]
extern crate error_chain;

use std::path::Path;
pub use errors::*;

mod errors;
mod named_tree;
mod fs_tree;

struct Config {
    root_dir: Box<Path>,
    block_size: i32,
}

quick_main!(|| -> Result<()> {
    Ok(())
});

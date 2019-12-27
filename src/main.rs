// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

// Import the macro.
#[macro_use]
extern crate error_chain;

// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
    }
}

// This only gives access within this module. Make this `pub use errors::*;`
// instead if the types must be accessible from other modules (e.g., within
// a `links` section).
use errors::*;
use std::path::Path;
use std::vec::Vec;

mod named_tree;
mod fs_tree;

fn chunk_buffer(contents: Box<[u8]>, _config: &Config)
                         -> Vec<Box<[u8]>> {
    //config.block_size
    return vec![contents];
}

struct Config {
    root_dir: Box<Path>,
    block_size: i32,
}

fn main() {
    let config = Config {
        root_dir: Box::from(Path::new("./")),
        block_size: 8192,
    };

    // Tree::construct(&*config.root_dir).unwrap();
}

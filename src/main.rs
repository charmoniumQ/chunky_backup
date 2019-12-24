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
        foreign_links {
            Io(std::io::Error);
            StripPrefix(std::path::StripPrefixError);
            WalkDir(walkdir::Error);
        }
    }
}

// This only gives access within this module. Make this `pub use errors::*;`
// instead if the types must be accessible from other modules (e.g., within
// a `links` section).
use errors::*;


use std::path::Path;
use std::vec::Vec;
use std::collections::HashMap;
use std::ffi::{OsStr, OsString};

struct Tree {
    path: Box<Path>,
    children: HashMap<OsString, Tree>,
}

impl Tree {
    fn insert(&mut self, child: &Path) -> Result<&mut Tree> {
        let rel_path = child.strip_prefix(&*self.path)?;
        self._insert_helper(
            rel_path.ancestors()
                .map(Box::from)
                .collect::<Vec<Box<Path>>>()
        )
    }

    fn _insert_helper(&mut self, mut ancestors: Vec<Box<Path>>)
                      -> Result<&mut Tree> {
        match ancestors.pop() {
            Some(ancestor) => {
                let name: &OsStr = ancestor.file_name()
                    .ok_or("No filename for this segment")?;
                self.children.entry(OsString::from(name))
                    .or_insert_with(|| Tree::new(ancestor))
                    ._insert_helper(ancestors)
            },
            None => Ok(self),
        }
    }

    fn new(path: Box<Path>) -> Tree {
        Tree {
            path,
            children: HashMap::new(),
        }
    }
}

fn construct_tree(config: &Config) -> Result<Tree> {
    let mut root = Tree::new(config.root_dir.clone());
    let maybe_dir_entries = walkdir::WalkDir::new(config.root_dir.clone())
        .follow_links(false);
    for maybe_dir_entry in maybe_dir_entries {
        let dir_entry = maybe_dir_entry?;
        root.insert(dir_entry.path())?;
    }
    Ok(root)
}

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

    construct_tree(&config).expect("");
}

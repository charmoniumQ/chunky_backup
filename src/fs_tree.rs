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

use std::ffi::{OsStr, OsString};
use std::path::Path;

// impl<'parent> Tree<'parent> {
//     fn insert(&mut self, child: &Path) -> Result<&mut Tree<'parent>> {
//         let rel_path = child.strip_prefix(&*self.path)?;
//         self._insert_helper(
//             rel_path.ancestors()
//                 .map(Box::from)
//                 .collect::<Vec<Box<Path>>>()
//         )
//     }

//     fn _insert_helper(&mut self, mut ancestors: Vec<Box<Path>>)
//                       -> Result<&mut Tree<'parent>> {
//         match ancestors.pop() {
//             Some(ancestor) => {
//                 let name: &OsStr = ancestor.file_name()
//                     .ok_or("No filename for this segment")?;
//                 self.children.entry(OsString::from(name))
//                     .or_insert_with(|| Tree::_new(ancestor, Some(self)))
//                     ._insert_helper(ancestors)
//             },
//             None => Ok(self),
//         }
//     }

//     fn construct(path: &Path) -> Result<Tree<'parent>> {
//         let mut root = Self::_new(Box::from(path), None);
//         let maybe_dir_entries = walkdir::WalkDir::new(path)
//             .follow_links(false);
//         for maybe_dir_entry in maybe_dir_entries {
//             let dir_entry = maybe_dir_entry?;
//             root.insert(dir_entry.path())?;
//         }
//         Ok(root)
//     }
// }

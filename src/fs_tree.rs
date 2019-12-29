use crate::errors::*;
use crate::named_tree::Tree;
use globset::Glob;
use std::ffi::OsString;
use std::path::Path;

pub struct FsTree<Data>(Tree<OsString, Data>);

impl<Data> FsTree<Data> {
    // pub fn insert_subtree(&self, child: &Path) -> Result<Self> {
    //     self.insert_subtree_helper(
    //         child.ancestors()
    //             .map(Box::from)
    //             .collect::<Vec<Box<Path>>>()
    //     )
    // }

    // fn insert_subtree_helper(&self, mut ancestors: Vec<Box<Path>>) -> Result<Self> {
    //     match ancestors.pop() {
    //         Some(ancestor) => {
    //             let name = OsString::from(
    //                 ancestor.file_name()
    //                     .ok_or("No filename for this segment")?
    //             );
    //             Ok(FsTree(self.0.child(&name)?
    //                       .ok_or("No such child")?
    //             ).insert_subtree_helper(ancestors)?
    //             )
    //         },
    //         None => Ok(FsTree(self.0.clone())),
    //     }
    // }

    pub fn construct(
        path: &Path,
        data_fn: impl Fn(&Path) -> Data,
        excludes: Vec<&str>,
    ) -> Result<Self> {
        let exclude_globs: Vec<_> = excludes
            .into_iter()
            .filter_map(|pattern| {
                Glob::new(pattern).map_or_else(
                    // If a glob fails, issue a non-fatal error
                    |err| {
                        eprintln!("Failed to parse \"{:?}\": {:?}", pattern, err);
                        None
                    },
                    Some,
                )
            })
            .map(|pattern| pattern.compile_matcher())
            .collect();
        let mut root = Tree::new(data_fn(path));
        let maybe_dir_entries = walkdir::WalkDir::new(path)
            .contents_first(false)
            .follow_links(false)
            .into_iter()
            .filter_entry(|p| exclude_globs.iter().any(|g| g.is_match(p.path())));
        for maybe_dir_entry in maybe_dir_entries {
            let dir_entry = maybe_dir_entry?;
            let path = dir_entry.path();
            let name = OsString::from(path.file_name().ok_or("no file name")?);
            root.insert(name, data_fn(path));
        }
        Ok(FsTree(root))
    }
}

use crate::errors::*;
use crate::named_tree::Tree;
use globset::Glob;
use std::path::Path;

impl<Data> Tree<String, Data> {
    pub fn construct(
        root_path: &Path,
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

        let root = Self::new(data_fn(root_path.strip_prefix(root_path)?));

        let maybe_dir_entries = walkdir::WalkDir::new(root_path)
            .contents_first(false)
            .follow_links(false)
            .into_iter()
            .filter_entry(|dir_entry| !exclude_globs.iter().any(|g| g.is_match(dir_entry.path())));

        for maybe_dir_entry in maybe_dir_entries {
            let dir_entry = maybe_dir_entry?;
            if dir_entry.path() != root_path {
                let path = dir_entry.path().strip_prefix(root_path)?;
                let name = path
                    .file_name()
                    .ok_or("no filename")?
                    .to_str()
                    .ok_or("not unicode")?
                    .to_owned();
                let parents = path
                    .parent()
                    .ok_or("file out of tree returned")?
                    .ancestors()
                    .filter_map(|p| p.file_name())
                    .map(|p| -> Result<_> { Ok(p.to_str().ok_or("not unicode")?.to_owned()) })
                    .collect::<Result<Vec<_>>>()?;
                root.recursive_get(parents)
                    .ok_or("violated my assumption that the fs will be traversed in pre-order")?
                    .insert(name, data_fn(path));
            }
        }

        Ok(root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io;

    fn touch(path: &Path) -> io::Result<fs::File> {
        fs::OpenOptions::new().create(true).write(true).open(path)
    }

    fn mkdirp(path: &Path) -> io::Result<()> {
        fs::create_dir_all(path)
    }

    #[test]
    fn it_works() -> Result<()> {
        use tempfile::tempdir;

        // Create a directory inside of `std::env::temp_dir()`.
        let dir = tempdir()?;
        let p = dir.path();

        mkdirp(&p.join("./a"))?;
        mkdirp(&p.join("./b"))?;
        touch(&p.join("./a/123"))?;
        touch(&p.join("./b/456"))?;
        mkdirp(&p.join("./b/789"))?;
        touch(&p.join("./b/789/abc"))?;
        touch(&p.join("./c"))?;

        let tree = Tree::construct(
            &p,
            |p| match p.file_name() {
                Some(f) => f.to_string_lossy().len(),
                None => 0,
            },
            vec![],
        )?;
        println!("{}", tree);
        println!("{:?}", tree);

        drop(&dir);
        dir.close()?;
        Ok(())
    }
}

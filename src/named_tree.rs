// We'll put our errors in an `errors` module, and other modules in
// this crate will `use errors::*;` to get access to everything
// `error_chain!` creates.
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {
        foreign_links {
            BorrowError(std::cell::BorrowError);
            BorrowMutError(std::cell::BorrowMutError);
        }
    }
}

// This only gives access within this module. Make this `pub use errors::*;`
// instead if the types must be accessible from other modules (e.g., within
// a `links` section).
use errors::*;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::hash::Hash;
use std::fmt::Debug;
use std::vec::Vec;

// For inspiriation, See
// https://github.com/SimonSapin/rust-forest/blob/master/rctree/lib.rs

struct TreeNode<Name, Data> {
    data: Data,
    parent: Option<WeakTreeLink<Name, Data>>,
    children: HashMap<Name, StrongTreeLink<Name, Data>>,
}

type StrongTreeLink<Name, Data> = Rc<RefCell<TreeNode<Name, Data>>>;
type WeakTreeLink<Name, Data> = Weak<RefCell<TreeNode<Name, Data>>>;

pub struct Tree<Name, Data>(StrongTreeLink<Name, Data>);

impl<Name, Data> Clone for Tree<Name, Data> {
    fn clone(&self) -> Tree<Name, Data> {
        Tree(self.0.clone())
    }
}

impl<Name: Hash + Eq + Debug + Clone, Data: Clone> Tree<Name, Data> {
    /**
Creates a new rooted tree.
     */
    pub fn new_root(data: Data) -> Self {
        Tree(Rc::new(RefCell::new(TreeNode {
            data,
            parent: None,
            children: HashMap::new(),            
        })))
    }

    /**
Create child named `name` with `default_data` if none exists.
     */
    pub fn ensure_child(&mut self, name: Name, default_data: Data) -> Result<Self> {
        Ok(Tree(self.0.try_borrow_mut()?
                .children
                .entry(name).or_insert_with(|| Rc::new(RefCell::new(TreeNode {
                    data: default_data,
                    parent: Some(Rc::downgrade(&self.0)),
                    children: HashMap::new(),
                })))
                .clone()
        ))
    }

    /**
Either creates or replaces `data` at the child named `name`.
     */
    fn set_child(&mut self, name: Name, data: Data) -> Result<Self> {
        let mut child_tree = self.ensure_child(name, data.clone())?;
        child_tree.0.try_borrow_mut()?.data = data;
        Ok(child_tree)
    }

    /**
Returns the child with name.
     */
    fn child(&self, name: &Name) -> Result<Option<Self>> {
        Ok(self.0.try_borrow()?
           .children
           .get(name)
           .map(|c| Tree(c.clone()))
        )
    }


    /**
Returns the parent, if one exists.
     */
    fn parent(&self) -> Result<Option<Self>> {
        Ok(match &self.0.try_borrow()?.parent {
            Some(parent) => Some(Tree(
                parent.clone()
                    .upgrade()
                    .ok_or("Parent dropped; Memory error I guess.")?
            )),
            None => None,
        })
    }

    fn data(&self) -> Result<Data> {
        Ok(self.0.try_borrow()?
           .data
           .clone())
    }

    fn children(&self) -> Result<Vec<(Name, Self)>> {
        Ok(self.0.try_borrow()?
           .children
           .iter()
           .map(|(name, tree)| (name.clone(), Tree(tree.clone())))
           .collect())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let mut tree: Tree<String, i16> = Tree::new_root(34);
        tree.ensure_child("bob".to_string(), 13)?;
        tree.ensure_child("bill".to_string(), 19)?;
        tree.ensure_child("bob".to_string(), 11)?;
        assert!(tree.parent()?.is_none());
        let mut children: Vec<String> = tree.children()?
            .into_iter()
            .map(|(name, _tree)| name)
            .collect();
        children.sort();
        assert_eq!(children, vec!["bill", "bob"]);
        assert_eq!(
            tree.child(&"bob".to_string())?
                .ok_or("missing child")?
                .parent()?
                .ok_or("missing parent")?
                .data()?,
            34
        );
        assert_eq!(
            tree.child(&"bob".to_string())?
                .ok_or("missing child")?
                .data()?,
            13
        );
        assert_eq!(
            tree.child(&"bill".to_string())?
                .ok_or("missing child")?
                .data()?,
            19
        );
        assert!(tree.child(&"joe".to_string())?.is_none());
        Ok(())
    }
}

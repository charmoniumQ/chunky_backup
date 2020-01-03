use crate::errors::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Result as fmtResult};
use std::hash::Hash;
use std::rc::{Rc, Weak};
use std::vec::Vec;

// For inspiriation, See
// https://github.com/SimonSapin/rust-forest/blob/master/rctree/lib.rs

struct TreeNode<Name, Data> {
    data: Rc<RefCell<Data>>,
    parent: Option<Weak<RefCell<Self>>>,
    children: HashMap<Name, Rc<RefCell<Self>>>,
}

pub struct Tree<Name, Data>(Rc<RefCell<TreeNode<Name, Data>>>);

impl<Name, Data> Clone for Tree<Name, Data> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<Name: Hash + Eq, Data> Tree<Name, Data> {
    /**
    Creates a new rooted tree.
         */
    pub fn new(data: Data) -> Self {
        Self(Rc::new(RefCell::new(TreeNode {
            data: Rc::new(RefCell::new(data)),
            parent: None,
            children: HashMap::new(),
        })))
    }

    /**
    Create child named `name` with a rooted subtree.
         */
    pub fn insert_subtree(&mut self, name: Name, subtree: Self) -> Self {
        subtree.0.borrow_mut().parent = Some(Rc::downgrade(&self.0));
        self.0.borrow_mut().children.insert(name, subtree.0.clone());
        subtree
    }

    /**
    Create child named `name` with `data`.
         */
    pub fn insert(&mut self, name: Name, data: Data) -> Self {
        let child_tree = Rc::new(RefCell::new(TreeNode {
            data: Rc::new(RefCell::new(data)),
            parent: Some(Rc::downgrade(&self.0)),
            children: HashMap::new(),
        }));
        let children = &mut self.0.borrow_mut().children;
        assert!(!children.contains_key(&name));
        children.insert(name, child_tree.clone());
        Self(child_tree)
    }

    /**
    Returns the child with name.
         */
    pub fn child(&self, name: &Name) -> Option<Self> {
        self.0.borrow().children.get(name).map(|c| Self(c.clone()))
    }

    pub fn recursive_get(&self, mut names: Vec<Name>) -> Option<Self> {
        match names.pop() {
            Some(name) => self.child(&name).and_then(|p| p.recursive_get(names)),
            None => Some(Self(self.0.clone())),
        }
    }
}

impl<Name, Data> Tree<Name, Data> {
    /**
    Returns the parent, if one exists.
         */
    pub fn parent(&self) -> Option<Self> {
        match &self.0.borrow().parent {
            Some(parent) => Some(Self(
                parent
                    .clone()
                    .upgrade()
                    .expect("Parent dropped; Memory error I guess."),
            )),
            None => None,
        }
    }

    pub fn data(&self) -> Rc<RefCell<Data>> {
        self.0.borrow().data.clone()
    }
}

impl<Name: Clone, Data> Tree<Name, Data> {
    pub fn children(&self) -> Vec<(Name, Self)> {
        self.0
            .borrow()
            .children
            .iter()
            .map(|(name, tree)| (name.clone(), Tree(tree.clone())))
            .collect()
    }
}

impl<Name: Debug + Clone, Data: Debug + Clone> Debug for Tree<Name, Data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        write!(f, "({:?} ", self.0.borrow().data.clone().borrow())?;
        for (name, subtree) in self.0.borrow().children.iter() {
            write!(f, "{:?} -> {:?} ", name, Tree(subtree.clone()))?;
        }
        write!(f, ")")?;
        Ok(())
    }
}

impl<Name: Display + Clone, Data: Display + Clone> Tree<Name, Data> {
    fn fmt_indent(&self, f: &mut Formatter<'_>, indent: usize, name: String) -> fmtResult {
        write!(
            f,
            "{}+ {} ({})",
            "-".repeat(indent),
            name,
            self.0.borrow().data.borrow()
        )?;
        for (name, subtree) in self.0.borrow().children.iter() {
            Self(subtree.clone()).fmt_indent(f, indent + 2, format!("{}", name))?;
        }
        Ok(())
    }
}

impl<Name: Display + Clone, Data: Display + Clone> Display for Tree<Name, Data> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmtResult {
        self.fmt_indent(f, 0, ".".to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<()> {
        let mut tree: Tree<String, i16> = Tree::new(34);
        tree.insert("bob".to_string(), 13);
        tree.insert("bill".to_string(), 19);
        assert!(tree.parent().is_none());
        let mut children: Vec<String> = tree
            .children()
            .into_iter()
            .map(|(name, _tree)| name.clone())
            .collect();
        children.sort();
        assert_eq!(children, vec!["bill", "bob"]);
        assert_eq!(
            tree.child(&"bob".to_string())
                .ok_or("missing child")?
                .parent()
                .ok_or("missing parent")?
                .data()
                .borrow()
                .clone(),
            34
        );
        assert_eq!(
            tree.child(&"bob".to_string())
                .ok_or("missing child")?
                .data()
                .borrow()
                .clone(),
            13
        );
        assert_eq!(
            tree.child(&"bill".to_string())
                .ok_or("missing child")?
                .data()
                .borrow()
                .clone(),
            19
        );
        assert!(tree.child(&"joe".to_string()).is_none());
        Ok(())
    }
}

#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct ThingsTree {
    pub children_of: Map<String, Vec<String>>, // parent -> Children
    pub parent_of: Map<String, String>,        // child -> parent
}

pub trait IntoAsciiTree {
    fn into_ascii_tree(&self, root: &str) -> ascii_tree::Tree;
    fn into_ascii_tree_string(&self, root: &str) -> String {
        let mut s = String::new();
        let _ = ascii_tree::write_tree(&mut s, &self.into_ascii_tree(root));
        s
    }
    fn into_ascii_forest(&self) -> String;
}

impl std::string::ToString for ThingsTree {
    /// Let's dump whole forest into String
    fn to_string(&self) -> String {
        let mut output = String::new();
        for (n, parent) in self.parent_of.iter() {
            if parent == "" {
                let root = n;
                let _ = ascii_tree::write_tree(&mut output, &self.into_ascii_tree(&root));
            }
        }
        output
    }
}

impl ThingsTree {
    pub fn new() -> ThingsTree {
        ThingsTree {
            children_of: Map::new(),
            parent_of: Map::new(),
        }
    }
    //TODO fn move() {}
    //TODO fn add_new() {}

    /// If child alredy has parent, then removes it from this parent
    fn remove_from_parent(&mut self, child: &str) {
        match self.parent_of.get(child) {
            Some(old_parent) => {
                //let &mut children_of_old_parent = self.children_of.get_mut(old_parent).unwrap();
                match self.children_of.get_mut(old_parent) {
                    Some(children_of_old_parent) => {
                        let child_position = children_of_old_parent
                            .iter()
                            .position(|s| *s == child)
                            .expect("This parent should have this child");
                        children_of_old_parent.swap_remove(child_position);
                    }
                    None => {}
                }
            }
            None => {}
        }
        self.parent_of.insert(child.to_string(), "".to_string());
    }

    /// parent may be "", what means "no parent"
    pub fn insert(&mut self, parent: &str, child: &str) {
        self.remove_from_parent(child);

        // We store information that node has no parent as Empty "" String.
        self.parent_of.insert(child.to_string(), parent.to_string());

        self.children_of
            .entry(parent.to_string())
            .or_insert(Vec::new())
            .push(child.to_string());

        // if parent is new to tree,
        // then let's register it has no parent with empty string
        self.parent_of
            .entry(parent.to_string())
            .or_insert("".to_string());
    }
}

impl IntoAsciiTree for ThingsTree {
    fn into_ascii_tree(&self, root: &str) -> ascii_tree::Tree {
        let children_names: &Vec<String> = match self.children_of.get(root) {
            Some(children) => children,
            None => return ascii_tree::Tree::Leaf(vec![root.to_string()]),
        };
        let children_trees = children_names
            .iter()
            .map(|kid| self.into_ascii_tree(kid))
            .collect();
        ascii_tree::Tree::Node(root.to_string(), children_trees)
    }
    fn into_ascii_forest(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod things_tree {
    use super::*;
    use ascii_tree::write_tree;
    use pretty_assertions::assert_eq;
    use std::error::Error;

    #[test]
    fn things_tree_test_simple_tree_00() -> Result<(), Box<dyn Error>> {
        let mut t = ThingsTree::new();
        t.insert("A", "B");
        t.insert("A", "C");
        t.insert("A", "D");
        t.insert("C", "E");
        t.insert("C", "F");
        let mut output = String::new();
        let at = t.into_ascii_tree("A");
        write_tree(&mut output, &at)?;
        assert_eq!(
            output.trim(),
            "
 A
 ├─ B
 ├─ C
 │  ├─ E
 │  └─ F
 └─ D"
                .trim()
        );
        Ok(())
    }

    #[test]
    fn things_tree_test_moving_nodes_00() -> Result<(), Box<dyn Error>> {
        let mut t = ThingsTree::new();
        t.insert("A", "B");
        t.insert("A", "C");
        let mut output = String::new();
        let at = t.into_ascii_tree("A");
        write_tree(&mut output, &at)?;
        assert_eq!(
            output.trim(),
            "
 A
 ├─ B
 └─ C"
                .trim()
        );
        // Let's move!
        t.insert("B", "C"); // as C is already under A it should be moved
        let mut output = String::new();
        let at = t.into_ascii_tree("A");
        write_tree(&mut output, &at)?;
        assert_eq!(
            output.trim(),
            "
 A
 └─ B
    └─ C"
                .trim()
        );

        Ok(())
    }

    #[test]
    fn things_tree_test_forest_to_string_00() -> Result<(), Box<dyn Error>> {
        let mut t = ThingsTree::new();
        t.insert("A", "B");
        t.insert("A", "C");
        t.insert("X", "Y");
        t.insert("X", "Z");
        t.insert("W", "V");

        let mut output = t.to_string();

        assert_eq!(
            output.trim(),
            "
 A
 ├─ B
 └─ C
 W
 └─ V
 X
 ├─ Y
 └─ Z
 "
            .trim()
        );
        Ok(())
    }
}

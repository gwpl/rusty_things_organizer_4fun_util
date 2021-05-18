#![allow(dead_code)]
use std::collections::BTreeMap as Map;

pub struct ThingsTree {
    pub children_of: Map<String, Vec<String>>, // parent -> Children
    pub parent_of: Map<String, String>,        // child -> parent
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
        self.parent_of.remove(child);
    }

    /// parent may be "", what means "no parent"
    pub fn add(&mut self, parent: &str, child: &str) {
        // TODO use swap_remove() on Vec
        // and https://stackoverflow.com/a/44012406/544721 for position
        self.remove_from_parent(child);
        if parent != "" {
            self.parent_of.insert(child.to_string(), parent.to_string());
        }
        self.children_of
            .entry(parent.to_string())
            .or_insert(Vec::new())
            .push(child.to_string());
    }

    pub fn into_ascii_tree(&self, root: &str) -> ascii_tree::Tree {
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
        t.add("A", "B");
        t.add("A", "C");
        t.add("A", "D");
        t.add("C", "E");
        t.add("C", "F");
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
        t.add("A", "B");
        t.add("A", "C");
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
        t.add("B", "C"); // as C is already under A it should be moved
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
}

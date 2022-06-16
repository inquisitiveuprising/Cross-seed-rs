use std::{collections::HashMap, slice::Iter, iter::Peekable};

use figment::value::{Value, Dict};

#[derive(Debug)]
pub enum ArgumentTreeNode {
    Leaf(Value),
    Branch(HashMap<String, ArgumentTreeNode>),
}

impl ArgumentTreeNode {
    /// Insert a value into the tree.
    pub fn insert(&mut self, keys: &mut Peekable<&mut Iter<String>>, value: Value) {
        let key = keys.next().unwrap();

        match self {
            ArgumentTreeNode::Leaf(_) => panic!("Cannot insert into a leaf node"),
            ArgumentTreeNode::Branch(children) => {
                match children.get_mut(key) {
                    // If the key is already in the tree, insert into it,
                    // going farther until the key doesn't exist anymore which
                    // is where we insert the value.
                    Some(node) => {
                        node.insert(keys, value);
                    }
                    None => {
                        // Check if we should insert a leaf or a branch by
                        // `peek`ing to see if there is a next key.
                        if keys.peek().is_none() {
                            let node = ArgumentTreeNode::Leaf(value);
                            children.insert(key.to_owned(), node);
                        } else {
                            let mut node = ArgumentTreeNode::Branch(HashMap::new());
                            node.insert(keys, value);

                            children.insert(key.to_owned(), node);
                        }
                    }
                }
            }
        }
    }

    /// Convert the tree into a `Dict`.
    fn to_dict(&self) -> Dict {
        match self {
            ArgumentTreeNode::Leaf(_) => panic!("Cannot convert a leaf node to a dict!"),
            ArgumentTreeNode::Branch(children) => {
                let mut dict = Dict::new();

                // Iterate over the children and recursively convert them to `Dict`s.
                // When it run into a leaf node, insert its value.
                for (key, node) in children.iter() {
                    match node {
                        ArgumentTreeNode::Leaf(value) => {
                            dict.insert(key.to_owned(), value.clone());
                        }
                        _ => {
                            dict.insert(key.to_owned(), Value::from(node.to_dict()));
                        }
                    }
                }
                dict
            }
        }
    }
}

#[derive(Debug)]
pub struct ArgumentTree {
    pub root: ArgumentTreeNode
}

impl ArgumentTree {
    /// Create a new tree.
    pub fn new() -> ArgumentTree {
        ArgumentTree {
            root: ArgumentTreeNode::Branch(HashMap::new())
        }
    }

    /// Insert into the tree.
    pub fn insert(&mut self, keys: &mut Iter<String>, value: Value) {
        let mut keys = keys.peekable();

        self.root.insert(&mut keys, value);
    }

    /// Convert the tree into a `Dict`.
    pub fn to_dict(&mut self) -> Dict {
        self.root.to_dict()
    }
}
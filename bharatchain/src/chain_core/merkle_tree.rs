use serde::{Deserialize, Serialize};
use sha3::{Digest, Keccak256};
use std::collections::HashMap;
use std::fmt;

// Define the types of nodes in the tree
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Node {
    Leaf {
        key: String,
        value: String,
    },
    Extension {
        prefix: String,
        child: String,
    },
    Branch {
        children: [Option<String>; 16], // this stores niggle of 16 char long hash
        value: Option<String>,
    },
}

pub struct MerklePatriciaTree {
    root: String,                 // The root hash of the tree
    nodes: HashMap<String, Node>, // Store nodes by their hash
}

impl MerklePatriciaTree {
    // Create a new, empty Merkle Patricia Tree
    pub fn new() -> Self {
        MerklePatriciaTree {
            root: String::new(),
            nodes: HashMap::new(),
        }
    }

    // Insert a key-value pair into the Merkle Patricia Tree
    pub fn insert(&mut self, key: String, value: String) {
        let node_hash = self.insert_node(&self.root, &key, &value);
        self.root = node_hash;
    }

    fn insert_node(&mut self, node_hash: &str, key: &str, value: &str) -> String {
        if node_hash.is_empty() {
            return self.create_leaf(key, value);
        }

        let node = self.nodes.get(node_hash).unwrap().clone();

        match node {
            Node::Leaf {
                key: node_key,
                value: node_value,
            } => {
                if node_key == *key {
                    return self.create_leaf(key, value);
                } else {
                    let new_branch = self.create_branch(node_key, node_value, key, value);
                    self.create_hash(new_branch)
                }
            }
            Node::Extension { prefix, child } => {
                if key.starts_with(&prefix) {
                    let child_hash = self.insert_node(&child, &key[prefix.len()..], value);
                    let new_extension = Node::Extension {
                        prefix,
                        child: child_hash,
                    };
                    return self.create_hash(new_extension);
                } else {
                    let new_branch = self.create_branch(value, new_key, new_value)
                    return self.create_hash(new_branch);
                }
            }
            Node::Branch {
                mut children,
                value: branch_value,
            } => {
                let idx = key.chars().next().unwrap_or('0') as usize;
                let child_hash =
                    self.insert_node(&children[idx].as_deref().unwrap_or(""), &key[1..], value);
                children[idx] = Some(child_hash);
                let new_branch = Node::Branch {
                    children,
                    value: Some(value.to_string()),
                };
                self.create_hash(new_branch)
            }
        }
    }

    // Helper function to create a leaf node (key-value pair)
    fn create_leaf(&mut self, key: &str, value: &str) -> String {
        let leaf = Node::Leaf {
            key: key.to_string(),
            value: value.to_string(),
        };
        self.create_hash(leaf)
    }

    // Create a branch node (with 16 possible children)
    fn create_branch(&mut self, value: String, new_key: &str, new_value: &str) -> Node {
        let mut children = [
            None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            None, None,
        ];
        children[0] = Some(self.create_leaf(new_key, new_value));
        Node::Branch {
            children,
            value: Some(value),
        }
    }

    // Generate a hash for a node (using Keccak256)
    fn create_hash(&mut self, node: Node) -> String {
        let serialized_node = serde_json::to_string(&node).unwrap();
        let hash = Keccak256::digest(serialized_node.as_bytes());
        let hash_str = format!("{:x}", hash);
        self.nodes.insert(hash_str.clone(), node);
        hash_str
    }

    // Retrieve a value for a given key from the tree
    pub fn get(&self, key: &str) -> Option<String> {
        self.get_node(&self.root, key)
    }

    fn get_node(&self, node_hash: &str, key: &str) -> Option<String> {
        let node = self.nodes.get(node_hash)?;

        match node {
            Node::Leaf {
                key: node_key,
                value,
            } => {
                if node_key == key {
                    Some(value.clone())
                } else {
                    None
                }
            }
            Node::Extension { prefix, child } => {
                if key.starts_with(prefix) {
                    self.get_node(child, &key[prefix.len()..])
                } else {
                    None
                }
            }
            Node::Branch { children, value } => {
                let idx = key.chars().next().unwrap_or('0') as usize;
                match &children[idx] {
                    Some(child_hash) => self.get_node(child_hash, &key[1..]),
                    None => value.clone(),
                }
            }
        }
    }
}

impl fmt::Debug for MerklePatriciaTree {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Root Hash: {}", self.root)
    }
}

// Main function to demonstrate usage
fn main() {
    let mut tree = MerklePatriciaTree::new();

    // Insert key-value pairs
    tree.insert("key1".to_string(), "value1".to_string());
    tree.insert("key2".to_string(), "value2".to_string());

    // Retrieve value for a key
    if let Some(value) = tree.get("key1") {
        println!("Found value: {}", value);
    } else {
        println!("Key not found");
    }

    // Display the root hash of the tree
    println!("{:?}", tree);
}

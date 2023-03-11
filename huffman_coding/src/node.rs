use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::{self, write, Display},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[repr(transparent)]
pub struct Tree(Node);

impl Tree {
    pub fn root(&self) -> &Node {
        &self.0
    }

    pub fn encoder<'a>(&'a self) -> Encoder {
        Encoder::new(self)
    }

    pub fn decode(&self, encoded: &str) -> String {
        let mut decoded = String::new();
        let mut current = self.root();
        for char in encoded.chars() {
            match char {
                '0' => current = current.left.as_ref().unwrap(),
                '1' => current = current.right.as_ref().unwrap(),
                _ => unreachable!(),
            }

            if let Some(key) = current.key {
                decoded.push(key);
                current = self.root();
            }
        }
        decoded
    }

    pub fn display(&self) -> NodeDisplay<'_> {
        self.root().display()
    }
}

impl<T> From<T> for Tree
where
    T: AsRef<str>,
{
    fn from(value: T) -> Self {
        fn inner(value: &str) -> Tree {
            let mut char_count = HashMap::new();
            for char in value.chars() {
                let entry = char_count.entry(char);
                *entry.or_insert(0) += 1;
            }

            println!("{char_count:?}");

            let mut nodes = char_count
                .iter()
                .map(|(char, count)| Node {
                    key: Some(*char),
                    count: *count,
                    left: None,
                    right: None,
                })
                .map(Reverse)
                .collect::<BinaryHeap<_>>();

            println!("-- Building the tree");
            while nodes.len() >= 2 {
                let left = nodes.pop().unwrap().0;
                let right = nodes.pop().unwrap().0;

                println!(
                    "popped left  '{:1}', {:3}",
                    left.key.unwrap_or(' '),
                    left.count
                );
                println!(
                    "popped right '{:1}', {:3}",
                    right.key.unwrap_or(' '),
                    right.count
                );

                let new_node = Node {
                    key: None,
                    count: left.count + right.count,
                    left: Some(Box::from(left)),
                    right: Some(Box::from(right)),
                };

                println!("new node     '{:1}', {:3}", ' ', new_node.count);

                nodes.push(Reverse(new_node));
            }

            Tree(nodes.pop().unwrap().0)
        }
        inner(value.as_ref())
    }
}

pub struct Encoder {
    bitmap: HashMap<char, String>,
}

impl Encoder {
    pub fn new(tree: &Tree) -> Self {
        println!("-- Building the bitmap");
        let mut bitmap = HashMap::new();
        let mut pending_nodes = vec![(tree.root(), String::new())];
        while let Some((node, bit_string)) = pending_nodes.pop() {
            if let Some(left) = &node.left {
                let mut left_string = bit_string.clone();
                left_string.push('0');
                pending_nodes.push((left, left_string));
            }

            if let Some(right) = &node.right {
                let mut right_string = bit_string.clone();
                right_string.push('1');
                pending_nodes.push((right, right_string));
            }

            if let Some(key) = node.key {
                println!("{key} = {bit_string}");
                bitmap.insert(key, bit_string);
            }
        }

        Self { bitmap }
    }

    pub fn encode(&self, word: &str) -> String {
        let mut encoded = String::new();
        for char in word.chars() {
            let bit_string = self.bitmap.get(&char).unwrap();
            encoded.push_str(bit_string);
        }
        encoded
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    pub key: Option<char>,
    pub count: usize,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

#[derive(Debug)]
pub struct NodeLeaf {
    pub key: char,
    pub count: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

impl Node {
    pub fn display(&self) -> NodeDisplay<'_> {
        NodeDisplay(self)
    }
}

pub struct NodeIter<'a> {
    pending: Vec<&'a Node>,
}

impl<'a> Iterator for NodeIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.pending.pop() {
            if let Some(left) = node.left.as_ref() {
                self.pending.push(left);
            }

            if let Some(right) = node.right.as_ref() {
                self.pending.push(right);
            }

            Some(node)
        } else {
            return None;
        }
    }
}

#[must_use]
pub struct NodeDisplay<'a>(&'a Node);

impl<'a> Display for NodeDisplay<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn write_child(f: &mut fmt::Formatter<'_>, node: &Node, depth: usize) -> fmt::Result {
            if depth > 1 {
                write!(f, "{}", " ".repeat((depth * 2) - 2))?;
            }

            if depth > 0 {
                write!(f, "|-")?;
            }

            match node.key {
                Some(key) => writeln!(f, "{} ({})", node.count, key),
                None => writeln!(f, "{}", node.count),
            }?;

            if let Some(node) = &node.left {
                write_child(f, node, depth + 1)?;
            }

            if let Some(node) = &node.right {
                write_child(f, node, depth + 1)?;
            }

            Ok(())
        }

        write_child(f, self.0, 0)
    }
}

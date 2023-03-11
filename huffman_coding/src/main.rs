mod node;

use crate::node::Tree;

fn main() {
    let word = "pneumonoultramicroscopicsilicovolcanoconiosis";
    let tree = Tree::from(word);

    println!("-- Tree");
    println!("{}", tree.display());

    println!("-- Encoded");
    let encoder = tree.encoder();
    let encoded = encoder.encode(word);
    println!("{encoded}");

    println!("-- Decoded");
    let decoded = tree.decode(&encoded);
    println!("{decoded}");

    println!("-- Compare");
    println!("word           = {} bytes", word.as_bytes().len());

    let encoded_len = (encoded.len() as f32 / 8.0).ceil() as usize;
    println!("encoded        = {} bytes", encoded_len);

    let tree_bytes = bincode::serialize(&tree).unwrap();
    println!("tree           = {} bytes", tree_bytes.len());
    println!("encoded + tree = {} bytes", encoded_len + tree_bytes.len())
}

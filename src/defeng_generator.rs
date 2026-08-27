use crate::cvtree::{CVTree, CVType};
use std::io::Write;

// Visits tree nodes and prints based on it.
pub fn generate(tree: &CVTree, c_formers: &Vec<String>, vs: &Vec<String>, c_latters: &Vec<String>, prefix: &String, file: &mut dyn Write) {
    if tree.depth == 0 {
        match tree.node {
            CVType::ConsonantFormer => {
                for c in c_formers.iter() {
                    let _ = write!(file, "{}", prefix);
                    let _ = writeln!(file, "{}", c);
                }
            },
            CVType::Vowel => {
                for v in vs.iter() {
                    let _ = write!(file, "{}", prefix);
                    let _ = writeln!(file, "{}", v);
                }
            },
            CVType::ConsonantLatter => {
                for c in c_latters.iter() {
                    let _ = write!(file, "{}", prefix);
                    let _ = writeln!(file, "{}", c);
                }
            }
        }
    } else {
        match tree.node {
            CVType::ConsonantFormer => {
                // One child
                let child = match &tree.children[0] {
                    Some(b) => { b.as_ref() },
                    None => panic!("Expected child.")
                };

                for c in c_formers {
                    let mut new_prefix = prefix.clone();
                    new_prefix.push_str(&c);
                    generate(child, c_formers, vs, c_latters, &new_prefix, file);
                }
            },
            CVType::Vowel => {
                // Two children
                let children = [
                    match &tree.children[0] {
                        Some(b) => { b.as_ref() },
                        None => panic!("Expected child.")
                    },
                    match &tree.children[1] {
                        Some(b) => { b.as_ref() },
                        None => panic!("Expected child.")
                    }
                ];

                for v in vs.iter() {
                    let mut new_prefix = prefix.clone();
                    new_prefix.push_str(&v);
                    generate(&children[0], c_formers, vs, c_latters, &new_prefix, file);
                    generate(&children[1], c_formers, vs, c_latters, &new_prefix, file);
                }
            },
            CVType::ConsonantLatter => {
                // One child
                let child = match &tree.children[0] {
                    Some(b) => { b.as_ref() },
                    None => panic!("Expected child.")
                };

                for c in c_latters.iter() {
                    let mut new_prefix = prefix.clone();
                    new_prefix.push_str(&c);
                    generate(child, c_formers, vs, c_latters, &new_prefix, file);
                }

            }
        }
    }

}

use std::env;
mod cvtree;

fn main() {
    let args: Vec<String> = env::args().collect();

    let ctree: cvtree::CVTree = cvtree::CVTree::new(2, cvtree::CVType::ConsonantFormer);

    println!("{ctree:?}");
}

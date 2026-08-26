mod cvtree;

fn main() {
    let ctree: cvtree::CVTree = cvtree::CVTree::new(2, cvtree::CVType::ConsonantFormer);

    println!("{ctree:?}");
}

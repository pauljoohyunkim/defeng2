use clap::Parser;
mod cvtree;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short='c', long)]
    consonant_former_file: String,

    #[arg(short='v', long)]
    vowel_file: String,

    #[arg(short='C', long)]
    consonant_latter_file: String,

    #[arg(short='m', long, default_value_t = 2)]
    min: u8,

    #[arg(short='M', long, default_value_t = 4)]
    max: u8,
}

fn main() {
    let args = Args::parse();
    
    let c_former_filename = args.consonant_former_file;
    let v_former_filename = args.vowel_file;
    let c_latter_filename = args.consonant_latter_file;
    let min_len = args.min;
    let max_len = args.max;

    let ctree: cvtree::CVTree = cvtree::CVTree::new(2, cvtree::CVType::ConsonantFormer);

    println!("{ctree:?}");
}

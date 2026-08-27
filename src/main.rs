use clap::Parser;
use std::fs::read_to_string;

mod cvtree;
mod defeng_generator;
use defeng_generator::generate;

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

fn read_lines(filename: &String) -> Vec<String> {
    read_to_string(filename)
    .unwrap()
    .lines()
    .map(String::from)
    .collect()
}

fn main() {
    let args = Args::parse();
    
    // Get necessary arguments.
    let c_former_filename = args.consonant_former_file;
    let v_filename = args.vowel_file;
    let c_latter_filename = args.consonant_latter_file;
    let min_len = args.min;
    let max_len = args.max;

    // Get cluster list.
    let c_formers = read_lines(&c_former_filename);
    let c_latters = read_lines(&c_latter_filename);
    let vs = read_lines(&v_filename);

    for i in min_len..(max_len+1) {
        // depth-1 = length
        let ctree = cvtree::CVTree::new(i as u16 - 1, cvtree::CVType::ConsonantFormer);
        let vtree = cvtree::CVTree::new(i as u16 - 1, cvtree::CVType::Vowel);

        let empty_str = String::from("");
        generate(&ctree, &c_formers, &vs, &c_latters, &empty_str);
        generate(&vtree, &c_formers, &vs, &c_latters, &empty_str);
    }
}

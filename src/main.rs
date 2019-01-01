use std::path::Path;

mod network;
mod parser;

fn main() {
    let network = network::new();

    println!("My network: {:?}", network);

    let path = Path::new("test_data/basic.md");
    parser::parse(path);
}
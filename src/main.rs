mod network;

fn main() {
    let network = network::create_empty_network();

    println!("My network: {:?}", network);
}
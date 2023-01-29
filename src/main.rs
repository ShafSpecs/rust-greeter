use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    key: String,
    
    #[clap(short, long)]
    value: String,
}

fn main() {
    let key = std::env::args().nth(1).expect("Expected a key");
    let value = std::env::args().nth(2).expect("Expected a value");

    println!("{} is the key, {} is the value", key, value);
}

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    name: String,
    
    #[clap(short, long)]
    count: u8,
}

fn main() {
    let args = Args::parse();

   for _ in 0..args.count {
       println!("Hello {}!", args.name)
   }
}

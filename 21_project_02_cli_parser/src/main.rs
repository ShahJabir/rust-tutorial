use clap::{Parser};

#[derive(Parser,  Debug)]
struct Args {
  #[arg(long, short)]
  input: String
}
fn main() {
    let args = Args::parse();
    println!("{:?}",args.input);
}

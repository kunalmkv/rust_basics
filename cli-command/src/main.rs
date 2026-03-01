use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long, short)]
    name: String,
}

fn main() {
    let args: Args = Args::parse();
    println!("Hi, {:?}", args);
}
use clap::Parser;
use std::fs;
use rustls::run;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser)]
    path: String,
}

fn main() {
    let cli = Cli::parse();
    
    match fs::read_dir(cli.path) {
        Ok(entries) => {
            run(entries);
        },
        Err(e) => println!("Error: {}", e),
    };
    
}

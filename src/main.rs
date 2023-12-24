use clap::Parser;
use std::fs;

/// My Simple Program
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// List of input values
    #[clap(value_parser)]
    path: String,
}

fn main() {
    let cli = Cli::parse();
    
    match fs::read_dir(cli.path) {
        Ok(entries) => {
            entries
                .filter_map(Result::ok) 
                .for_each(|entry| {
                    println!("{:?}", entry.path());
                });
        },
        Err(e) => println!("Error: {}", e),
    };
    
    
}

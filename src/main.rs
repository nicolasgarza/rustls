use clap::Parser;

/// My Simple Program
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// List of input values
    #[clap(value_parser)]
    inputs: Vec<String>,
}

fn main() {
    let cli = Cli::parse();

    for input in cli.inputs {
        println!("Looking for files in path: {}", input);
    }
}

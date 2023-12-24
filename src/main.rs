use clap::Parser;
use std::fs;


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
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path().to_string_lossy().to_string();
                println!("Path: {:?}", path);

                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        println!("File extension: {:?}", path.rsplit_once('.').map_or("", |(_, right)| right));
                        println!("{} bytes", metadata.len());
                    } else if metadata.is_dir() {
                        println!("Folder");
                        println!("Permissions: {:?}", metadata.permissions());
                    }
                }
            }
        },
        Err(e) => println!("Error: {}", e),
    };
    
    
}

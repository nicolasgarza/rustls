use clap::{command, arg, Command};
use std::fs;

fn main() {
    let matches = command!()
        .arg(arg!([path])
            .required(true)
            .index(1)
        )
        .get_matches();
        
    
    let path = matches.get_one::<String>("path");

        match fs::read_dir(path.unwrap()) {
            Ok(entries) => {
                rustls::run_ls(entries);
            },
            Err(e) => println!("Error: {}", e),
        };

        
}




use clap::Parser;
use std::fs::{self, DirEntry};
use std::time::SystemTime;
use chrono::{DateTime, Utc};

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
                println!("{}", construct_path_str(entry));
            }
        },
        Err(e) => println!("Error: {}", e),
    };
    
    
}

fn construct_path_str(entry: DirEntry) -> String {
    let path = entry.path().to_string_lossy().to_string();
    let mut fin: String = String::new();

    if let Ok(metadata) = entry.metadata() {
        if metadata.permissions().readonly() {
            fin = String::from("Read-only   ");
        } else {
            fin = String::from("Writable    ");
        }
        fin.push_str({
            &format_system_time(metadata.modified())
        });
        if metadata.is_file() {
            fin.push_str(metadata.len().to_string().as_str())
        } 
    };
    fin.push_str(path.as_str());
    fin
}


fn format_system_time(result: Result<SystemTime, std::io::Error>) -> String {
    match result {
        Ok(system_time) => {
            // Convert SystemTime to DateTime<Utc>
            let datetime: DateTime<Utc> = system_time.into();

            // Format the DateTime into the desired string format
            datetime.format("%m/%d/%Y %I:%M %p").to_string()
        }
        Err(_) => {
            // Handle the error case, perhaps by returning a default string or an error message
            "Error: Unable to retrieve time".to_string()
        }
    }
}
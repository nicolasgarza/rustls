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
                construct_path_str(entry);
            }
        },
        Err(e) => println!("Error: {}", e),
    };
    
    
}

fn construct_path_str(entry: DirEntry) {
    let path = entry.path().to_string_lossy().to_string();
    let mut object_permissions = String::new(); 
    let mut last_modified_date = String::new(); 
    let mut file_size = String::new();
    
    if let Ok(metadata) = entry.metadata() {

        object_permissions = if metadata.permissions().readonly() {
            "Read-only".to_string()
        } else {
            "Writable".to_string()
        };

        if let Ok(time) = metadata.modified() {
            last_modified_date = format_system_time(time);
        }

        if metadata.is_file() {
            file_size = metadata.len().to_string();
        }
    }
    println!("{}", print_object(path, object_permissions, last_modified_date, file_size))
    
}

fn print_object(path: String, object_permissions: String, last_modified_date: String, file_size: String) -> String{
    let formatted_perms = format!("{:<width$}", object_permissions, width = 14);
    let formatted_mod_date = format!("{:>width$}", last_modified_date, width = 20);
    let formatted_size = format!("{:>width$}", file_size, width = 14);
    let formatted_path = format!("{:>width$}", path, width=30);
    format!("{}{}{}{}", formatted_perms, formatted_mod_date, formatted_size, formatted_path)
}

fn format_system_time(result: SystemTime) -> String {
    let datetime: DateTime<Utc> = result.into();

    datetime.format("%m/%d/%Y %I:%M %p").to_string()
}
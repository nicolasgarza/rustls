// ls

use std::fmt;
use std::fs::DirEntry;
use std::time::SystemTime;
use chrono::{DateTime, Utc};

pub struct Config {
    pub long_format: bool,
    pub display_all: bool,
    pub recursively_list: bool,
    pub sort_by_modify_date: bool,
    pub sort_by_file_size: bool,
    pub reverse_order: bool,
}

struct File {
    name: String,
    permissions: Option<String>,
    last_modified_date: Option<String>,
    file_size: Option<String>,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut parts = Vec::new();
        parts.push(self.name.clone());
        match &self.permissions {
            Some(value) => parts.push(format!("{:12}", value)),
            None => parts.push("            ".to_string()),
        };
        match &self.last_modified_date {
            Some(value) => parts.push(format!("{:20}", value)),
            None => parts.push("                    ".to_string()),
        };
        match &self.file_size {
            Some(value) => parts.push(format!("{:12}", value)),
            None => parts.push("            ".to_string()),
        };
        write!(f, "{}", parts.join(" "))
    }
}

pub fn run_ls(path: Option<&String>, config: Config) {
    match config.recursively_list {
        true => recursive_walk(path.unwrap(), config),
        false => walk(path.unwrap(), config),
    }
}

pub fn walk(path: &String, config: Config) {
    let mut files: Vec<File> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if config.long_format {
                    files.push(long_format_file(entry));
                } else {
                    files.push(File {
                        name: entry.file_name().to_string_lossy().to_string(),
                        permissions: None,
                        last_modified_date: None,
                        file_size: None,
                    });
                }
            }
        }
    }
    
    println!("{}", format_files(files, config));
    
}

pub fn recursive_walk(_path: &String, _config: Config) {
    todo!()
}

fn long_format_file(entry: DirEntry) -> File {
    let path = entry.file_name().to_string_lossy().to_string();
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
    
    File {
        name: path,
        permissions: Some(object_permissions),
        last_modified_date: Some(last_modified_date),
        file_size: Some(file_size),
    }
    
}

fn format_system_time(result: SystemTime) -> String {
    let datetime: DateTime<Utc> = result.into();

    datetime.format("%m/%d/%Y %I:%M %p").to_string()
}

fn format_files(mut files: Vec<File>, config: Config) -> String {
    let max_name_length = files.iter().map(|item| item.name.len()).max().unwrap_or(0);
    for file in files.iter_mut() {
        file.name = format!("{:width$}", file.name, width = max_name_length+5);
    }

    if !config.display_all {
        files = files.into_iter().filter(|item| !item.name.starts_with(".")).collect();
    }

    if config.sort_by_modify_date {
        files.sort_by(|a,b| a.last_modified_date.cmp(&b.last_modified_date));
    } else if config.sort_by_file_size {
        files.sort_by(|a,b| a.file_size.cmp(&b.file_size));
    }

    if config.reverse_order {
        files.reverse();
    }
    if config.long_format {
        files.insert(0, File {
            name: format!("{:width$}", "Name", width = max_name_length+5),
            permissions: Some("Permissions".to_string()),
            last_modified_date: Some("Last Modified".to_string()),
            file_size: Some("Size".to_string()),
        });    
    } else {
        files.insert(0, File {
            name: "Name".to_string(),
            permissions: None,
            last_modified_date: None,
            file_size: None,
        });
    }
    
    files.iter().map(|item| item.to_string()).collect::<Vec<_>>().join("\n")
}
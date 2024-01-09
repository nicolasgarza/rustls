use clap::{command, Arg, arg, ArgAction};
use rustls::Config;

fn main() {
    let matches = command!()
        .arg(arg!([path])
            .required(true)
            .index(1)
        )
        .arg(Arg::new("long_format")
            .short('l')
            .long("long-format")
            .action(ArgAction::SetTrue)
            .help("displays in long format"))
        .arg(Arg::new("display_all_files")
            .short('a')
            .long("display-all")
            .action(ArgAction::SetTrue)
            .help("Displays all files in directory, even hidden"))
        .arg(Arg::new("recursively_list")
            .short('R')
            .long("recursive-display")
            .action(ArgAction::SetTrue)
            .help("recursively lists subdirectories"))
        .arg(Arg::new("sort_by_modify_date")
            .short('t')
            .long("sort-by-modify-date")
            .action(ArgAction::SetTrue)
            .help("sort files by modify date")
            .conflicts_with("sort_by_file_size"))
        .arg(Arg::new("sort_by_file_size")
            .short('S')
            .long("sort-by-file-size")
            .action(ArgAction::SetTrue)
            .help("sort files by size")
            .conflicts_with("sort_by_modify_date"))
        .arg(Arg::new("reverse_order")
            .short('r')
            .long("reverse-order")
            .action(ArgAction::SetTrue)
            .help("Reverses order of output"))
        .get_matches();
        
    
    let path = matches.get_one::<String>("path");
    let config = Config {
        long_format: matches.get_flag("long_format"),
        display_all: matches.get_flag("display_all_files"),
        recursively_list: matches.get_flag("recursively_list"),
        sort_by_modify_date: matches.get_flag("sort_by_modify_date"),
        sort_by_file_size: matches.get_flag("sort_by_file_size"),
        reverse_order: matches.get_flag("reverse_order"),
    };

    rustls::run_ls(path, config);
        
}




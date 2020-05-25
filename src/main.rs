use std::process;

use clap::{App, Arg};

mod rename_file_utils;

fn main() {
    let matches = App::new("File renamer")
        .version("0.1.0")
        .author("Tom Weissinger")
        .about("Renames files for prep to upload to OneDrive")
        .arg(Arg::with_name("dir")
                 .short("d")
                 .long("dir")
                 .takes_value(true)
                 .help("Directory to do"))
        .arg(Arg::with_name("test")
                 .short("t")
                 .long("test")
                 .takes_value(false)
                 .help("Whether to test"))
        .get_matches();
    let dir = matches.value_of("dir").unwrap_or_else(|| {
        println!("Need to specify a directory");
        process::exit(1);
    });

    if matches.is_present("test") {
        rename_file_utils::prep_cleanup_file_names("testdir.templ", dir);
    }
    
    rename_file_utils::check_frequency(dir);
    rename_file_utils::cleanup_file_names(dir);
    rename_file_utils::check_frequency(dir);
}
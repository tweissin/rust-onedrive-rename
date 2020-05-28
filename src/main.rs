use std::env;
use std::io;
use std::process;

use clap::{App, Arg};

type Error = Box<dyn std::error::Error>;
type Result<T, E = Error> = std::result::Result<T, E>;

mod auth;
mod rename_file_utils;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("File renamer")
        .version("0.1.0")
        .author("Tom Weissinger")
        .about("Renames files for prep to upload to OneDrive")
        .arg(Arg::with_name("dir")
                 .short("d")
                 .long("dir")
                 .takes_value(true)
                 .help("Directory on which to perform renaming"))
        .arg(Arg::with_name("test")
                 .short("t")
                 .long("test")
                 .takes_value(false)
                 .help("Whether to test"))
        .arg(Arg::with_name("check")
                 .short("c")
                 .long("check")
                 .takes_value(false)
                 .help("Whether to just do a dry-run check but not change anything"))
        .arg(Arg::with_name("access")
            .short("a")
            .long("access")
            .takes_value(false)
            .help("Whether to print an access token"))
        .get_matches();
    let dir = matches.value_of("dir").unwrap_or_else(|| {
        println!("Need to specify a directory");
        process::exit(1);
    });
    if matches.is_present("access") {
        let tenant = env::var("TENANT").unwrap();
        let client_id = env::var("CLIENT_ID").unwrap();
        let client_secret = env::var("CLIENT_SECRET").unwrap();
        let access_token = auth::get_access_token(tenant, client_id, client_secret).await;
        println!("{}", access_token.unwrap());
        process::exit(1);
    }
    if matches.is_present("test") {
        let do_it = check_yes("This will delete files, type yes to continue!");
        if do_it {
            rename_file_utils::prep_cleanup_file_names("testdir.templ", dir);
        } else {
            println!("Aborted");
            process::exit(1);
        }
    }
    if matches.is_present("check") {
        println!("NOTE: Only running check on files.");
        rename_file_utils::check_frequency(dir);
    }
    rename_file_utils::check_frequency(dir);
    rename_file_utils::cleanup_file_names(dir);
    rename_file_utils::check_frequency(dir);
    
    Ok(())
}

fn check_yes(message: &str) -> bool {
    println!("{}", message);
    let mut response = String::new();
    io::stdin().read_line(&mut response).expect("Failed to read line");
    return response.trim().to_lowercase() == "yes";
}
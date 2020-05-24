mod rename_file_utils;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    // rename_file_utils::check_frequency("testdir.templ");
    // rename_file_utils::prep_cleanup_file_names("testdir.templ", "testdir");
    // rename_file_utils::cleanup_file_names("testdir");
    // rename_file_utils::check_frequency("testdir");
    Ok(())
}
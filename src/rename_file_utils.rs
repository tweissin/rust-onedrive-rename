extern crate fs_extra;

use std::borrow::Borrow;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use copy_dir;
use fs_extra::dir::{copy, CopyOptions};
use regex::Regex;
use walkdir::WalkDir;

// template_dir should have this structure:
// + testdir.templ
//   - 1st_:\<>-*"?.rtf
//   + subdir
//     - 2nd_:\<>-*"?.rtf
//     + subsubdir
//       - 3rd_:\<>-*"?.rtf
pub fn prep_cleanup_file_names(template_dir: &str, output_dir: &str) {
    if Path::new(output_dir).exists() {
        println!("removing dir {}", output_dir);
        fs::remove_dir_all(output_dir);
    }
    println!("copying dir {} to {}", template_dir, output_dir);
    let options = CopyOptions::new();
    copy_dir::copy_dir(template_dir, output_dir);
}

struct Report {
    frequency: HashMap<char, i32>
}

impl Report {
    fn new() -> Report {
        Report {
            frequency: HashMap::new()
        }
    }

    fn record(&mut self, ch: char) {
        if !self.frequency.contains_key(&ch) {
            self.frequency.insert(ch, 1);
        } else {
            let mut val = self.frequency.get(&ch);
            let val = val.unwrap() + 1 as i32;
            self.frequency.insert(ch, val);
        }
    }

    fn print_report(&self) {
        for (k, v) in self.frequency.iter() {
            println!("{:?}: {:?}", k, v);
        }
    }
}

pub fn check_frequency(target_dir: &str) {
    println!("Checking this directory {} please wait", target_dir);

    let mut report = Report::new();
    let invalid_chars = vec!['/', '\\', '<', '>', ':', '*', '\"', '?', '|'];
    for entry in WalkDir::new(target_dir) {
        let ent = entry.unwrap();
        let path = ent.path().display().to_string();
        let pos = path.rfind('/');
        if pos == None || ent.path().is_dir() {
            continue
        }
        let filename = &path[pos.unwrap()+1..];

        for ch in invalid_chars.iter() {
            let str = ch.to_string();
            if filename.contains(&str) {
                report.record(*ch);
            }
        }
    }
    println!("Printing invalid character frequency");
    report.print_report()
}

pub fn cleanup_file_names(target_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    /*
      / \ < > : * " ? |
    */
    let re = Regex::new(r#"[:\\"?<>\\\*\|]"#).unwrap();

    for entry in WalkDir::new(target_dir) {
        let ent = entry.unwrap();
        let path = ent.path().display().to_string();
        let pos = path.rfind('/');
        if pos == None || ent.path().is_dir() {
            continue
        }
        let dirname = &path[0..pos.unwrap()];
        let filename = &path[pos.unwrap()+1..];

        if !re.is_match(filename) {
            continue
        }
        let new_filename = re.replace_all(filename, "-");
        let old_filename = [dirname, "/", &filename].concat();
        let new_filename = [dirname, "/", &new_filename].concat();

        println!("Renaming {} to {}", &old_filename, &new_filename);
        fs::rename(&old_filename, &new_filename);
    }

    Ok(())
}
extern crate fs_extra;

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use copy_dir;
use regex::Regex;
use walkdir::WalkDir;
use std::collections::HashSet;

struct Report {
    frequency: HashMap<char, i32>,
    offending_files: HashSet<String>
}

impl Report {
    fn new() -> Report {
        Report {
            frequency: HashMap::new(),
            offending_files: HashSet::new()
        }
    }

    fn record(&mut self, path: &str, ch: char) {
        if !self.frequency.contains_key(&ch) {
            self.frequency.insert(ch, 1);
        } else {
            let val = self.frequency.get(&ch);
            let val = val.unwrap() + 1 as i32;
            self.frequency.insert(ch, val);
        }
        self.record_offending_file(path);
    }

    fn record_offending_file(&mut self, path: &str) {
        self.offending_files.insert(path.to_string());
    }

    fn print_report(&self) {
        println!("Printing invalid character frequency");
        if self.frequency.len()==0 {
            println!(" - None!");
        }

        for (k, v) in self.frequency.iter() {
            println!(" {:?}: {:?}", k, v);
        }

        println!("Printing all offending files");
        if self.offending_files.len()==0 {
            println!(" - None!");
        }

        for file in &self.offending_files {
            println!(" {:?}", file);
        }
    }
}

pub fn check_frequency(target_dir: &str) {
    if !Path::new(target_dir).exists() {
        println!("Path doesn't exist: {}", target_dir);
    }

    println!("Checking this directory: '{}'", target_dir);

    let mut report = Report::new();
    let invalid_chars = vec!['/', '\\', '<', '>', ':', '*', '\"', '?', '|'];
    let re_space = Regex::new(r"(^\s+)|(\s+$)").unwrap();

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
                report.record(&path, *ch);
            }
        }

        if re_space.is_match(filename) {
            report.record_offending_file(&path);
        }
    }
    report.print_report()
}

// This cleans up filenames by replacing unsupported characters with "-"
// OneDrive doesn't like these characters in filenames:
//  / \ < > : * " ? |
pub fn cleanup_file_names(target_dir: &str) {
    if !Path::new(target_dir).exists() {
        println!("Path doesn't exist: {}", target_dir);
    }

    println!("Processing files");
    let re = Regex::new(r#"[:\\"?<>\\\*\|]"#).unwrap();
    let re_space = Regex::new(r"(^\s+)|(\s+$)").unwrap();

    for entry in WalkDir::new(target_dir) {
        let ent = entry.unwrap();
        let path = ent.path().display().to_string();
        let pos = path.rfind('/');
        if pos == None {
            continue
        }
        let dirname = &path[0..pos.unwrap()];
        let filename = &path[pos.unwrap()+1..];

        if !re.is_match(filename) && !re_space.is_match(filename) {
            continue
        }
        let new_filename = re.replace_all(filename, "-");
        let new_filename = new_filename.trim();
        let old_filename = [dirname, "/", &filename].concat();
        let new_filename = [dirname, "/", &new_filename].concat();

        println!(" Renaming {} to {}", &old_filename, &new_filename);
        let res = fs::rename(&old_filename, &new_filename);
        match res {
            Ok(file) => file,
            Err(error) => println!("Problem renaming file: {:?}", error),
        };
    }
}

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
        let res = fs::remove_dir_all(output_dir);
        match res {
            Ok(file) => file,
            Err(error) => panic!("Problem renaming file: {:?}", error),
        };
    }
    println!("copying dir {} to {}", template_dir, output_dir);
    let res = copy_dir::copy_dir(template_dir, output_dir);
    match res {
        Ok(file) => file,
        Err(error) => panic!("Problem copying directory: {:?}", error)
    };
}


use std::io::BufRead;
use std::path::{PathBuf, Path};
use ignore::{Walk};
use std::io::{BufReader};
use std::fs::{File};

fn main() {
	let walker = Walk::new("./").into_iter();
    for entry in walker {
    	match entry {
    		Ok(entry) => {
    			if let Some(file_type) = entry.file_type() {
    				if file_type.is_file() {
    					parse_todos(entry.path());	
    				}
    			}
    		},
    		Err(error) => {
    			println!("Error: {:?}", error);
    			return
    		}
    	}
    }
}

fn parse_todos(file_path: &Path) {
	let file_name = match file_path.file_name() {
		Some(f) => f,
		None => return,
	};

	let file = match File::open(file_path) {
		Ok(f) => f,
		Err(e) => {
			println!("Encountered error {:?} trying to open file", e);
			return
		}
	};
	let reader = BufReader::new(file);
	
	// TODO: parse todos here
	// TODO: test this on this file
	let mut line_number = 0;

	for line in reader.lines() {
		line_number += 1;
		match line {
			Ok(line_content) => {
				if line_content.contains("TODO") {
					println!("File name: {:#?} at line {}: \n {} \n", file_name, line_number, line_content.trim());
				}
			} 
			Err(e) => {
				println!("Encountered error while reading line: {:?}", e);
				continue
			}
		}
	}
}
use std::io::BufRead;
use std::path::{PathBuf};
use ignore::{Walk};
use std::io::{BufReader};
use std::fs::{File};

fn main() {
	let walker = Walk::new("./").into_iter();
    for entry in walker {
    	match entry {
    		Ok(entry) => {
    			parse_todos(entry.into_path());
    		},
    		Err(error) => println!("Error: {:?}", error)
    	}
    }
}

fn parse_todos(file_path: PathBuf) {
	let file = match File::open(file_path) {
		Ok(f) => f,
		Err(e) => {
			println!("Encountered error {:?} trying to open file", e);
			return
		}
	};
	let reader = BufReader::new(file);
	for line in reader.lines() {
		println!("{:?}", line);
	}
}
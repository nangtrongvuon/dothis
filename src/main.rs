extern crate clap;

use std::io::BufRead;
use std::env;
use std::path::{Path};
use ignore::{Walk};
use std::io::{BufReader};
use std::fs::{File};

struct TodoSearcher<'a> {
	trigger_words: Vec<&'a str>
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

const USAGE: &str = "
Usage: dothis [path]
";

fn main() {	
	let mut args = Vec::new();

	args.push(env::args().next().expect("there should be at least one path"));
	for arg in env::args().skip(1) {
		args.push(arg);
	}

	let arguments: Vec<&str> = args.iter().map(String::as_str).collect();

	if arguments.contains(&"VERSION") {
		println!("DoThis version: {}", VERSION);
		return
	}

	if arguments.contains(&"help") {
		println!("{}", USAGE);
		return
	}

	let walker = Walk::new("./").into_iter();
	let some_trigger_words = vec!["TODO", "dungle:"];
	let todo_searcher = TodoSearcher::new(some_trigger_words);

    for entry in walker {
    	match entry {
    		Ok(entry) => {
    			if let Some(file_type) = entry.file_type() {
    				if file_type.is_file() {
    					todo_searcher.parse_todos(entry.path());	
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

impl<'a> TodoSearcher<'a> {
	fn new(trigger_words: Vec<&'a str>) -> Self {
		TodoSearcher {
			trigger_words: trigger_words
		}
	}

	fn parse_todos(&self, file_path: &Path) {
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

		// dungle: try using line
		for line in reader.lines() {
			line_number += 1;
			match line {
				Ok(line_content) => {
					for word in &self.trigger_words {
						if line_content.contains(word) && line_content.trim().starts_with("//") {
							println!("File name: {:#?} at line {}: \n {} \n", file_name, line_number, line_content.trim());
						}	
					}
				} 
				Err(e) => {
					println!("Encountered error while reading line: {:?}", e);
					continue
				}
			}
		}
	}
}


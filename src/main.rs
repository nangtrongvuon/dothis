extern crate clap;

use clap::{App, Arg};
use ignore::Walk;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

struct TodoSearcher<'a> {
    trigger_words: Vec<&'a str>,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = App::new("DoThis")
        .version(VERSION)
        .author("Dzung Le <leviethoangdung@outlook.com>")
        .about("Checks files for todo markers")
        .arg(
            Arg::with_name("project_path")
                .takes_value(true)
                .required(true)
                .help("The root path of the project to search"),
        )
        .arg(
            Arg::with_name("trigger_words")
                .short("t")
                .long("trigger")
                .takes_value(false)
                .help("The words to search for. Default is `TODO`."),
        )
        .get_matches();

    let path_str = matches.value_of("project_path").expect("path should exist");
    let path = Path::new(path_str);
    let trigger_words_list: Vec<&str>;

    if let Some(trigger_words) = matches.value_of("trigger_words") {
        trigger_words_list = trigger_words.split_whitespace().collect();
    } else {
        trigger_words_list = vec!["TODO"];
    }

    let todo_searcher = TodoSearcher::new(trigger_words_list);
    todo_searcher.perform_search(&path);
}

impl<'a> TodoSearcher<'a> {
    fn new(trigger_words: Vec<&'a str>) -> Self {
        TodoSearcher { trigger_words }
    }

    fn perform_search(&self, main_path: &Path) {
        let walker = Walk::new(main_path);

        for entry in walker {
            match entry {
                Ok(entry) => {
                    if let Some(file_type) = entry.file_type() {
                        if file_type.is_file() {
                            self.parse_todos(entry.path());
                        }
                    }
                }
                Err(error) => {
                    println!("Error: {:?}", error);
                    return;
                }
            }
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
                return;
            }
        };
        let reader = BufReader::new(file);
        // TODO: parse todos here
        // TODO: test this on this file

        // dungle: try using line
        for (line_number, line) in reader.lines().enumerate() {
            match line {
                Ok(line_content) => {
                    if line_content.trim().contains("//") {
                        for word in &self.trigger_words {
                            if line_content.contains(word) {
                                println!(
                                    "{} | Line {}: \n {} \n",
                                    file_name.to_str().expect("Valid file name"),
                                    line_number,
                                    line_content.trim()
                                );
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Encountered error {:?} reading file: {:?}", e, file_path);
                    return;
                }
            }
        }
    }
}

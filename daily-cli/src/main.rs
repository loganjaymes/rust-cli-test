use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, prelude::*};
use chrono::{Local};

struct Config {
    tasks: Vec<String>,
}

struct DailyFile {
    file_path: String,
    checklist: HashMap<String, bool>,
}


fn start_up() -> String {
    let uname = users_native::get_current_username();
    println!("Good afternoon, {}", uname);
    
    let now = Local::now();
    let formatted_date = now.format("%Y-%m-%d");
    let formatted_time = now.format("%H:%M:%S");
    let today = now.format("%Y%m%d");
    println!("It is currently {} at {}", formatted_date, formatted_time);

    today.to_string()
}

fn load_file() /* -> Result<Config, io::Error > */{
    println!("Locating file...");

    let paths = fs::read_dir("./days").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.extension().and_then(OsStr::to_str) == Some("lg") {
            println!("{}", path.display());
        }
    }
    println!("Would you like to use one of the above files or create a new one? (if no files were listed, type 'new').");

    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let new = String::from("new");
    match input.trim() {
        new => {
            println!("What would you like to name the file? DO NOT ADD ANY EXTENSIONS!");
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            let mut new_file = format!("./days/{}.lg", input.trim());

            let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(new_file);
        },
        _ => {
            let mut opened_file = format!("./days/{}.lg", input.trim());
            let file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(input);
        }
    };
    // fs::read_to_string(path);
}
/*
fn try_open(fname: &String, td: String, c: Config) -> Result<io::Error, String> {

}
*/
fn main() {
    let today = start_up();
    load_file();
}

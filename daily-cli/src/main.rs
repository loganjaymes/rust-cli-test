use std::env;
use std::fs::File;
use std::io;
use chrono::{Local};

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

fn try_open() {
    
}

fn main() {
    let today = start_up();
    println!("What file would you like to open?");

    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Failed to read file name");

    if file_name.trim() == "today" {
        let mut folder_path = String::from("days/");
        folder_path.push_str(&today);
        println!("{folder_path}");

        let file_open_result = File::open(folder_path);

        let file_open = match file_open_result {
            Ok(file) => file,
            Err(error) => panic!("PROBLEM!! {error:?}"),
        };
    }
    // need: 
    // regex for checking if file of appropriate name exists
    //      if exists: open it and do asdjkkasldjas
    //      doesnt: create, then do asdjhaksd
    //
    // later:
    // config files for customizable tasks
    

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}

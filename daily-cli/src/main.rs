use std::fs;
use std::io;
use std::io::prelude::*;
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

fn try_open(fname: &String, td: String) -> (bool, String) {
    // TODO: make file struct that is just filled from here
    // stores stuff like pathname and prolly hashmap of task names and vals
    let mut folder_path = String::from("days/");

    if fname.trim() == "today" {
        folder_path.push_str(&td);
        // println!("{folder_path}");

        let mut file_open = fs::File::open(&folder_path).unwrap_or_else(|error| {
            if error.kind() == io::ErrorKind::NotFound {
                fs::File::create(&folder_path).unwrap_or_else(|error| {
                    panic!("Problem creating file: {error:?}");
                })
            } else {
                panic!("Problem opening file: {error:?}");
            }
        });

        let _ = file_open.write_all(b"task1,task2,task3,task4\nfalse,false,false,false");

        return (true, folder_path)

    } else { // TODO: validate string to ensure its in YYYYMMDD format. ignore for now 
        folder_path.push_str(&fname.trim());
        // println!("{folder_path}");

        let mut file_open = fs::File::open(&folder_path).unwrap_or_else(|error| { // might be able to
                                                                               // just remove let
                                                                               // but idk yet
            if error.kind() == io::ErrorKind::NotFound {
                fs::File::create(&folder_path).unwrap_or_else(|error| {
                    panic!("Problem creating file: {error:?}");
                })
            } else {
                panic!("Problem opening file: {error:?}");
            }
        });


        let _ = file_open.write_all(b"task1,task2,task3,task4\nfalse,false,false,false");

        return (true, folder_path)
    }

     // return (false, String::from(""))
}

fn main() {
    let today = start_up();
    println!("What file would you like to open?");

    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).expect("Failed to read file name");
    let pair = try_open(&file_name, today);

    if pair.0 {
        // println!("File exists");
        let contents = fs::read_to_string(&pair.1).expect("Should read");
        println!("{}", contents);
    }

    /*
     * actually open file should be some shit like
     * task1 | task2 | task3 | task4...
     * true  | false | true  | true....
     */
}

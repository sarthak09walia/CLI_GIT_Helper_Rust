use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    let initial_path = get_initial_path();
    env::set_current_dir(&initial_path).expect("Failed to change directory");

    loop {
        println!(" ");
        println!("Git Manager - Please select an option:");
        println!("1. Initialize Repo");
        println!("2. Make Commit");
        println!("3. Clone a Repository");
        println!("0. Exit");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let option = input.trim().parse::<i32>().unwrap();

        match option {
            1 => {
                git_init();
            }
            2 => git_commit(),
            3 => {
                git_clone();
                
            }
            0 => {
                println!("Exiting...");
                break;
            }
            _ => println!("Invalid option"),
        }
    }
}

fn get_initial_path() -> String {
    println!("Please enter the initial directory path where you want to perform Git operations:");
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");
    path.trim().to_string()
}

fn git_init() {
   
}

fn git_clone(){}


fn git_commit() {
    // make commit
    println!("Made new commit");
}



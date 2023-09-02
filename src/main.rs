use std::env;
use std::io;
use std::process::Command;
use colored::Colorize;

fn main() {
    let initial_path = get_initial_path();
    env::set_current_dir(&initial_path).expect("Failed to change directory");

    loop {
        println!(" ");
        println!("Git Manager - Please select an option:");
        println!(" ");
        println!("1. Initialize Repo");
        println!("2. Make Commit");
        println!("3. Clone a Repository");
        println!("4. Get Git Status");
        println!("5. Git pull request");
        println!("0. Exit");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let option = input.trim().parse::<i32>().unwrap();

        match option {
            1 => git_init(),
            2 => git_commit(),
            3 => git_clone(),
            4 => git_status(),
            5 => git_pull(),
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
    // Execute the command and capture its output
    let output = Command::new("cmd")
        .args(["/C", "git init"])
        .output()
        .expect("failed to execute process");

    // Check if the command was successful
    if output.status.success() {
        // Convert the stdout bytes to a string
        if let Ok(stdout_str) = String::from_utf8(output.stdout) {
            println!("Command output: {}", stdout_str);
        } else {
            println!("Failed to git init");
        }
    } else {
        // Handle command failure
        let exit_code = output.status.code().unwrap_or(1); // Default to 1 if exit code is None
        eprintln!("Command failed with exit code: {}", exit_code);
    }
}

fn git_clone() {
    println!("Please enter github url to clone *only public repositories");
    let mut url = String::new();
    std::io::stdin().read_line(&mut url).unwrap();

    let output= Command::new("cmd")
                .args(["/C","git", "clone", &url])
                .output()
                .expect("Failed to clone");

    // Check if the command was successful
    if output.status.success() {
        // Convert the stdout bytes to a string
        if let Ok(stdout_str) = String::from_utf8(output.stdout) {
            println!("Clone Success {}", stdout_str);
        } else {
            println!("Failed to capture command output");
        }
    } else {
        // Handle command failure
        let exit_code = output.status.code().unwrap_or(1); // Default to 1 if exit code is None
        eprintln!("Command failed with exit code: {}", exit_code);
    }
}

fn git_commit() {
    // Execute the command and capture its output, adding files before committing
    let output = Command::new("cmd")
        .args(["/C", "git", "add", "."])
        .output()
        .expect("failed to execute process");

    // Check if the command was successful
    if output.status.success() {
        println!("Added files to git");
    } else {
        // Handle command failure
        let exit_code = output.status.code().unwrap_or(1); // Default to 1 if exit code is None
        eprintln!("Command failed with exit code: {}", exit_code);
    }

    //getting git commit message
    println!("Please enter commit message:");
    let mut message = String::new();
    std::io::stdin().read_line(&mut message).unwrap();

    // Remove leading and trailing whitespace from the message
    let message = message.trim();

    // Execute the command and capture its output
    let output = Command::new("cmd")
        .args(["/C", "git", "commit", "-m", message])
        .output()
        .expect("failed to execute process");

    // Check if the command was successful
    if output.status.success() {
        // Convert the stdout bytes to a string
        if let Ok(stdout_str) = String::from_utf8(output.stdout) {
            println!("Command output: {}", stdout_str);
        } else {
            println!("Failed to capture command output");
        }
    } else {
        // Handle command failure
        let exit_code = output.status.code().unwrap_or(1); // Default to 1 if exit code is None
        eprintln!("Command failed with exit code: {}", exit_code);
    }
}

fn git_status() {
    let output = Command::new("cmd")
        .args(["/C","git", "status"])
        .output()
        .expect("failed to get status");


    if output.status.success() {
        // Convert the stdout bytes to a string
        if let Ok(stdout_str) = String::from_utf8(output.stdout) {
            println!("{}", stdout_str);
        } else {
            println!("Failed to capture command output");
        }
    } else {
        // Handle command failure
        let exit_code = output.status.code().unwrap_or(1); // Default to 1 if exit code is None
        eprintln!("Command failed with exit code: {}", exit_code);
    }
}

fn git_pull(){
 println!("Enter branch name: ");

 let mut branch_name = String::new();
 io::stdin().read_line(&mut branch_name).unwrap();

 let output = Command::new("cmd").args(["/C","git","pull","origin", &branch_name]).output().expect("git pull FAILED");

 if output.status.success() {
    // Convert the stdout bytes to a string
    if let Ok(stdout_str) = String::from_utf8(output.stdout) {
        println!("{}", stdout_str);
    } else {
        println!("Failed to capture command output");
    }
} else {
    // Handle command failure
    let exit_code = output.status.code().unwrap_or(1); // Default to 1 if exit code is None
    eprintln!("Command failed with exit code: {}", exit_code);
}
}
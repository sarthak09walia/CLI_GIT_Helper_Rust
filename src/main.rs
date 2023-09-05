extern crate colored;
use std::env;
use std::io;
use std::process::Command;
use colored::*;
use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
};


fn main() {
    let initial_path = get_initial_path();
    env::set_current_dir(&initial_path).expect("Failed to change directory");

    loop {
        clear_screen();

        println!("Git Manager - Please select an option:\n");
        println!("1. Initialize Repo");
        println!("2. Make Commit");
        println!("3. Clone a Repository");
        println!("4. Get Git Status");
        println!("5. Git Pull Request");
        println!("0. Exit");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let option = input.trim().parse::<i32>().unwrap_or(-1);

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
    println!(
        "{}",
        "Please enter the initial directory path where you want to perform Git operations:".blue().bold()
    );
    let mut path = String::new();
    io::stdin()
        .read_line(&mut path)
        .expect("Failed to read line");
    path.trim().to_string()
}

fn clear_screen() {
    execute!(
        io::stdout(),
        terminal::Clear(ClearType::All), // Use ClearType::All to clear the terminal
        cursor::MoveTo(0, 0)
    )
    .expect("Failed to clear terminal");
}

fn git_init() {
    execute_command("git init");
    pause_for_effect();
}

fn git_clone() {
    println!("Please enter the GitHub URL to clone (public repositories only):");
    let mut url = String::new();
    io::stdin().read_line(&mut url).unwrap();
    execute_command(&format!("git clone {}", url.trim()));
    pause_for_effect();
}

fn git_commit() {
    execute_command("git add .");
    println!("Please enter commit message:");
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();
    let message = message.trim();
    execute_command(&format!("git commit -m '{}'", message));
    pause_for_effect();
}

fn git_status() {
    execute_command("git status");
    pause_for_effect();
}

fn git_pull() {
    println!("Enter branch name:");
    let mut branch_name = String::new();
    io::stdin().read_line(&mut branch_name).unwrap();
    let branch_name = branch_name.trim();
    execute_command(&format!("git pull origin {}", branch_name));
    pause_for_effect();
}

fn execute_command(command: &str) {
    let output = Command::new("cmd")
        .args(["/C", command])
        .output()
        .expect("Failed to execute process");

    if output.status.success() {
        if let Ok(stdout_str) = String::from_utf8(output.stdout) {
            println!("Command output: {}", stdout_str.green());
        } else {
            println!("Failed to capture command output");
        }
    } else {
        let exit_code = output.status.code().unwrap_or(1);
        eprintln!("Command failed with exit code: {}", exit_code);
    }
}

fn pause_for_effect() {
    std::thread::sleep(std::time::Duration::from_secs(2));
}


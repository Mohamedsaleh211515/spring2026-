use std::io;
use std::process::Command;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(path) => {
            let result = Command::new("ls")
                .arg(path)
                .status()
                .expect("Failed to execute ls");

            if result.success() {
                println!("Listed files successfully.");
            } else {
                println!("Error listing files.");
            }
        }

        FileOperation::Display(file) => {
            let result = Command::new("cat")
                .arg(file)
                .status()
                .expect("Failed to execute cat");

            if result.success() {
                println!("Displayed file.");
            } else {
                println!("Error displaying file.");
            }
        }

        FileOperation::Create(file, content) => {
            let command = format!("echo '{}' > {}", content, file);

            let result = Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .expect("Failed to create file");

            if result.success() {
                println!("File created successfully.");
            } else {
                println!("Error creating file.");
            }
        }

        FileOperation::Remove(file) => {
            let result = Command::new("rm")
                .arg(file)
                .status()
                .expect("Failed to remove file");

            if result.success() {
                println!("File removed successfully.");
            } else {
                println!("Error removing file.");
            }
        }

        FileOperation::Pwd => {
            let result = Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");

            if result.success() {
                println!("Printed working directory.");
            } else {
                println!("Error printing directory.");
            }
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        println!("Enter your choice (0-5): ");

        let choice = get_input();

        match choice.as_str() {
            "1" => {
                println!("Enter directory path:");
                let path = get_input();
                let op = FileOperation::List(path);
                perform_operation(op);
            }
            "2" => {
                println!("Enter file path:");
                let path = get_input();
                let op = FileOperation::Display(path);
                perform_operation(op);
            }
            "3" => {
                println!("Enter file path:");
                let path = get_input();

                println!("Enter content:");
                let content = get_input();

                let op = FileOperation::Create(path, content);
                perform_operation(op);
            }
            "4" => {
                println!("Enter file path:");
                let path = get_input();
                let op = FileOperation::Remove(path);
                perform_operation(op);
            }
            "5" => {
                let op = FileOperation::Pwd;
                perform_operation(op);
            }
            "0" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}
use std::io;
use std::io::Write;
use std::io::Read;
use std::process::Command;
use std::fs::File;
use std::path::Path;

#[derive(PartialEq)]
enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
    Exit,
    NoAction
}

const MENU: &'static str = "File Operations Menu
1. List files in a directory
2. Display file contents
3. Create a new file 
4. Remove a file
5. Print working directory
0. Exit 

$ ";

fn list_files(path: &String) {
    let path = Path::new(path.as_str());
    if !path.exists() {
        println!("Error: Path `{}` does not exist.", path.display());
        return;
    }
    
    if path.is_dir() {
        for entry in path.read_dir().unwrap() {
            print!("{} ", entry.unwrap().file_name().display());
        }
        println!();
    } else {
        println!("Error: Path `{}` is not a directory.", path.display());
    }
}

fn display_file(path: &String) {
    let path = Path::new(path.as_str());
    if !path.exists() {
        println!("Error: Path `{}` does not exist.", path.display());
        return;
    }

    if path.is_dir() {
        println!("Error: Path `{}` is a directory.", path.display());
        return;
    }

    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    
    println!("{}", contents);
}

fn create_file(path: &String, content: &String) {
    let path = Path::new(path.as_str());
    if path.exists() {
        println!("File `{}` already exists.", path.display());
        return;
    }

    let mut file = File::create(path).unwrap();
    file.write_all(content.as_bytes()).unwrap();
    println!("File `{}` created.", path.display());
}

fn remove_file(path: &String) {
    let path = Path::new(path.as_str());

    if path.exists() {
        if path.is_dir() {
            if !std::fs::read_dir(path).unwrap().next().is_none() {
                println!("Error: directory `{}` is not empty.", path.display());
            } else {
                std::fs::remove_file(path).unwrap();
                println!("Directory `{}` deleted.", path.display());
            }
        } else {
            std::fs::remove_file(path).unwrap();
            println!("File `{}` deleted.", path.display());
        }
    }
}

fn get_pwd() {
    Command::new("pwd").status().expect("Failed to execute pwd.");
}

fn perform_operation(proccode: &FileOperation) {
    match proccode {
        FileOperation::List(path) => list_files(&path),
        FileOperation::Display(path) => display_file(&path),
        FileOperation::Create(path, content) => create_file(&path, &content),
        FileOperation::Remove(path) => remove_file(&path),
        FileOperation::Pwd => get_pwd(),
        _ => ()
    }
}

fn main() {
    let mut proccode = FileOperation::NoAction;
    let stdin = io::stdin();
    let mut buffer = String::new();
    
    while (proccode != FileOperation::Exit) {
        print!("{}", MENU);
        io::stdout().flush();

        stdin.read_line(&mut buffer).unwrap();
        if buffer.chars().last().unwrap() == '\n' {
            buffer.pop();
        }

        let args: Vec<&str> = buffer.trim().split(' ').collect();
        
        if args.is_empty() || args[0] == "" {
            println!("Error: Command argument expected.");
            buffer.clear();
            continue;
        }

        proccode = match args[0] {
            "0" => FileOperation::Exit,
            "1" => match args.len() {
                1 => FileOperation::List(".".to_string()),
                2 => FileOperation::List(args[1].to_string()),
                _ => {
                    println!("Too many arguments!\nUsage: `1 [<path>]`");
                    FileOperation::NoAction
                }
            },
            "2" => match args.len() {
                2 =>  FileOperation::Display(args[1].to_string()),
                _ => {
                    println!("Invalid syntax.\nUsage: `2 <path>`");
                    FileOperation::NoAction
                }
            },
            "3" => match args.len() {
                1 => {
                    println!("Invalid syntax.\nUsage: `3 <path> <content>`");
                    FileOperation::NoAction
                },
                2 => {
                    println!("Invalid syntax.\nUsage: `3 <path> <content>`");
                    FileOperation::NoAction
                },
                _ => {
                    let mut content = String::new();
                    for s in args[1..].iter() {
                        content += s;
                        content += " ";
                    }

                    FileOperation::Create(args[1].to_string(), content)
                }
            },
            "4" => match args.len() {
                2 => FileOperation::Remove(args[1].to_string()),
                _ => {
                    println!("Invalid syntax.\nUsage: `4 <path>`");
                    FileOperation::NoAction
                }
            },
            "5" => FileOperation::Pwd,
            _ => {
                println!("Command not found.");
                FileOperation::NoAction
            },
        };

        perform_operation(&proccode);

        println!();
        buffer.clear();
    }
}

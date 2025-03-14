use clap::{Arg, Command};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    done: bool,
}

const FILE_PATH: &str = "tasks.json";

fn main() {
    let matches = Command::new("Task Manager")
        .version("1.0")
        .author("Nkwenti-Severian")
        .about("A simple CLI task manager")
        .subcommand(Command::new("add").arg(Arg::new("desc").required(true)))
        .subcommand(Command::new("list"))
        .subcommand(Command::new("done").arg(Arg::new("id").required(true)))
        .subcommand(Command::new("remove").arg(Arg::new("id").required(true)))
        .get_matches();

    let mut tasks = load_tasks();

    if let Some(matches) = matches.subcommand_matches("add") {
        let desc = matches.get_one::<String>("desc").unwrap().to_string();
        add_task(&mut tasks, desc);
    } else if matches.subcommand_matches("list").is_some() {
        list_tasks(&tasks);
    } else if let Some(matches) = matches.subcommand_matches("done") {
        let id: usize = matches.get_one::<String>("id").unwrap().parse().unwrap();
        mark_done(&mut tasks, id);
    } else if let Some(matches) = matches.subcommand_matches("remove") {
        let id: usize = matches.get_one::<String>("id").unwrap().parse().unwrap();
        remove_task(&mut tasks, id);
    }
}

fn load_tasks() -> Vec<Task> {
    if !Path::new(FILE_PATH).exists() {
        return Vec::new();
    }
    let mut file = File::open(FILE_PATH).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new())
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).expect("Failed to serialize");
    fs::write(FILE_PATH, json).expect("Failed to write file");
}

fn add_task(tasks: &mut Vec<Task>, description: String) {
    let id = tasks.len() + 1;
    tasks.push(Task {
        id,
        description,
        done: false,
    });
    save_tasks(tasks);
    println!("Task added successfully!");
}

fn list_tasks(tasks: &Vec<Task>) {
    if tasks.is_empty() {
        println!("No tasks available.");
        return;
    }
    for task in tasks {
        println!(
            "{} [{}] - {}",
            task.id,
            if task.done { "✓" } else { " " },
            task.description
        );
    }
}

fn mark_done(tasks: &mut Vec<Task>, id: usize) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.done = true;
        save_tasks(tasks);
        println!("Task marked as done!");
    } else {
        println!("Task not found!");
    }
}

fn remove_task(tasks: &mut Vec<Task>, id: usize) {
    if let Some(pos) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(pos);
        save_tasks(tasks);
        println!("Task removed successfully!");
    } else {
        println!("Task not found!");
    }
}

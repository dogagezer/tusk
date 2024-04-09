use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    io,
};

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Task {
    description: String,
    completed: bool,
}

impl Task {
    pub fn new(description: String) -> Self {
        Task {
            description,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }

    pub fn uncomplete(&mut self) {
        self.completed = false;
    }
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Account {
    name: String,
    tasks: Vec<Task>,
    subaccounts: HashMap<String, Account>,
}

impl Account {
    pub fn new(name: String) -> Self {
        Account {
            name,
            tasks: Vec::new(),
            subaccounts: HashMap::new(),
        }
    }

    pub fn add_task(&mut self, description: String) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    pub fn delete_task(&mut self, id: usize) {
        if id > 0 && id <= self.tasks.len() {
            self.tasks.remove(id - 1);
        }
    }

    pub fn complete_task(&mut self, id: usize) -> Result<(), &'static str> {
        if let Some(task) = self.tasks.get_mut(id - 1) {
            task.complete();
            Ok(())
        } else {
            Err("Invalid task index")
        }
    }

    pub fn uncomplete_task(&mut self, id: usize) -> Result<(), &'static str> {
        if let Some(task) = self.tasks.get_mut(id - 1) {
            task.uncomplete();
            Ok(())
        } else {
            Err("Invalid task index")
        }
    }

    pub fn clear_tasks(&mut self) {
        self.tasks.clear();
    }

    pub fn display_tasks(&self) {
        match self.tasks.len() {
            0 => println!("No tasks available for account '{}'!", self.name),
            _ => {
                println!("Tasks for account '{}':", self.name);
                self.tasks.iter().enumerate().for_each(|(index, task)| {
                    println!("{}. [{}] {}", index + 1, if task.completed { "X" } else { " " }, task.description);
                });
            }
        }
    }
}

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand, Debug)]
enum Command {
    Add { acc: String, description: String },
    List { acc: String },
    Delete { acc: String, id: usize },
    Complete { acc: String, id: usize },
    Uncomplete { acc: String, id: usize },
    Clear { acc: String },
}

fn load_tasks_from_file(filename: &str) -> Result<HashMap<String, Account>, io::Error> {
    // Attempt to read the file
    match read_to_string(filename) {
        Ok(contents) => {
            // If the file exists and has content, deserialize the data
            serde_json::from_str(&contents).map_err(Into::into)
        }
        Err(_) => {
            // If the file does not exist or cannot be read, return an empty HashMap
            Ok(HashMap::new())
        }
    }
}

fn save_tasks_to_file(filename: &str, accounts: &HashMap<String, Account>) -> Result<(), io::Error> {
    let serialized_data = serde_json::to_string(accounts)?;
    write(filename, serialized_data)?;
    Ok(())
}

fn handle_add_command(acc: &str, description: String, accounts: &mut HashMap<String, Account>) {
    accounts.entry(acc.to_string()).or_insert_with(|| Account::new(acc.to_string())).add_task(description);
    println!("Task added to account '{}'!", acc);
}

fn handle_delete_command(acc: &str, id: usize, accounts: &mut HashMap<String, Account>) {
    if let Some(account) = accounts.get_mut(acc) {
        account.delete_task(id);
        println!("Task deleted from account '{}'!", acc);
    } else {
        println!("No such account '{}'", acc);
    }
}

fn handle_list_command(acc: &str, accounts: &HashMap<String, Account>) {
    if let Some(account) = accounts.get(acc) {
        match account.tasks.len() {
            0 => println!("No tasks available for account '{}'!", acc),
            _ => {
                println!("Tasks for account '{}':", acc);
                account.tasks.iter().enumerate().for_each(|(index, task)| {
                    println!("{}. [{}] {}", index + 1, if task.completed { "X" } else { " " }, task.description);
                });
            }
        }
    } else {
        println!("Account '{}' not found. Please create it first.", acc);
    }
}

fn handle_complete_command(acc: &str, id: usize, accounts: &mut HashMap<String, Account>) {
    if let Some(account) = accounts.get_mut(acc) {
        match account.complete_task(id) {
            Ok(_) => {
                handle_list_command(acc, accounts);
            }
            Err(err) => println!("No such task: {}", err),
        }
    } else {
        println!("No such account '{}'", acc);
    }
}

fn handle_uncomplete_command(acc: &str, id: usize, accounts: &mut HashMap<String, Account>) {
    if let Some(account) = accounts.get_mut(acc) {
        match account.uncomplete_task(id) {
            Ok(_) => handle_list_command(acc, accounts),
            Err(err) => println!("No such task: {}", err),
        }
    } else {
        println!("No such account '{}'", acc);
    }
}

fn handle_clear_command(acc: &str, accounts: &mut HashMap<String, Account>) {
    if let Some(account) = accounts.get_mut(acc) {
        account.clear_tasks();
        println!("Cleared the account '{}'!", acc);
    } else {
        println!("No such account '{}'", acc);
    }
}

fn main() -> Result<(), io::Error> {
    let filename = "task_data.json";
    let mut accounts: HashMap<String, Account> = load_tasks_from_file(filename)?;

    let args = Cli::parse();
    match args.command {
        Some(Command::Add { description, acc }) => {
            handle_add_command(&acc, description, &mut accounts);
        }
        Some(Command::Delete { id, acc }) => {
            handle_delete_command(&acc, id, &mut accounts);
        }
        Some(Command::Complete { id, acc }) => {
            handle_complete_command(&acc, id, &mut accounts);
        }
        Some(Command::Uncomplete { id, acc }) => {
            handle_uncomplete_command(&acc, id, &mut accounts);
        }
        Some(Command::List { acc }) => {
            handle_list_command(&acc, &accounts);
        }
        Some(Command::Clear { acc }) => {
            handle_clear_command(&acc, &mut accounts);
        }
        _ => {
            let help_page = r#"
Welcome to TUSK!

This CLI app helps you manage your tasks across different accounts.

Usage:
    task_manager [SUBCOMMAND]

Subcommands:
    add       Add a new task to an account       tusk add --acc <account_name> --description "Task description"
    delete    Delete a task from an account      tusk delete --acc <account_name> --id <task_id>
    complete  Mark a task as completed           tusk complete --acc <account_name> --id <task_id>
    uncomplete    Mark a completed task as incomplete   tusk incomplete --acc <account_name> --id <task_id>
    list      List all tasks for an account      tusk list --acc <account_name>
    clear     Clear all tasks for an account     tusk clear --acc <account_name>


Enjoy managing your tasks efficiently with TUSK :)!
"#;
            println!("{}", help_page);
        }
    }
    save_tasks_to_file(filename, &accounts)?;
    Ok(())
}

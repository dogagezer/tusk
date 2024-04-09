use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    io,
};
use colored::Colorize;
use clap::{Parser, Subcommand};
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
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
    let contents = match read_to_string(filename) {
        Ok(contents) => contents,
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => {
            // If the file does not exist, return an empty HashMap
            return Ok(HashMap::new());
        }
        Err(err) => return Err(err),
    };

    // If the file exists and has content, deserialize the data
    serde_json::from_str(&contents).map_err(Into::into)
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
        let mut table = Table::new();
        table.load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(80)
            .set_header(vec![
                Cell::new("ID").fg(Color::Green),
                Cell::new("Status").fg(Color::Green),
                Cell::new("Description").fg(Color::Green),
            ]);

        for (index, task) in account.tasks.iter().enumerate() {
            let status = if task.completed { "X" } else { " " };
            let mut description_cell = Cell::new(task.description.clone());
            if !task.completed {
                description_cell = description_cell.add_attribute(Attribute::SlowBlink);
            }else { description_cell= description_cell.fg(Color::Green)}
            table.add_row(vec![
                Cell::new(format!("{}", index + 1)),
                Cell::new(status),
                description_cell,
            ]);
        }

        if table.is_empty() {
            println!("No tasks available for account '{}'!", acc);
        } else {
            println!("Tasks for account '{}':", acc);
            println!("{table}");
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
            let welcome = "🐘 Welcome to TUSK! 📝".yellow().bold();
            let usage = r#"
Usage:
    task_manager [SUBCOMMAND]
"#.green().bold();
            let mut table = Table::new();
            table
                .load_preset(UTF8_FULL)
                .apply_modifier(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::DynamicFullWidth)
                .set_width(120)
                .add_row(vec![
                    Cell::new("➕ add").fg(Color::Blue).set_alignment(CellAlignment::Left),
                    Cell::new("Add a new task to an account").set_alignment(CellAlignment::Left),
                    Cell::new("tusk add --acc <account_name> --description \"Task description\"").set_alignment(CellAlignment::Left),
                ])
                .add_row(vec![
                    Cell::new("🗑️ delete").fg(Color::Blue).set_alignment(CellAlignment::Left),
                    Cell::new("Delete a task from an account").set_alignment(CellAlignment::Left),
                    Cell::new("tusk delete --acc <account_name> --id <task_id>").set_alignment(CellAlignment::Left),
                ])
                .add_row(vec![
                    Cell::new("✅ complete").fg(Color::Blue).set_alignment(CellAlignment::Left),
                    Cell::new("Mark a task as completed").set_alignment(CellAlignment::Left),
                    Cell::new("tusk complete --acc <account_name> --id <task_id>").set_alignment(CellAlignment::Left),
                ])
                .add_row(vec![
                    Cell::new("❎ uncomplete").fg(Color::Blue).set_alignment(CellAlignment::Left),
                    Cell::new("Mark a completed task as incomplete").set_alignment(CellAlignment::Left),
                    Cell::new("tusk incomplete --acc <account_name> --id <task_id>").set_alignment(CellAlignment::Left),
                ])
                .add_row(vec![
                    Cell::new("📋 list").fg(Color::Blue).set_alignment(CellAlignment::Left),
                    Cell::new("List all tasks for an account").set_alignment(CellAlignment::Left),
                    Cell::new("tusk list --acc <account_name>").set_alignment(CellAlignment::Left),
                ])
                .add_row(vec![
                    Cell::new("🧹 clear").fg(Color::Blue).set_alignment(CellAlignment::Left),
                    Cell::new("Clear all tasks for an account").set_alignment(CellAlignment::Left),
                    Cell::new("tusk clear --acc <account_name>").set_alignment(CellAlignment::Left),
                ]);
            println!("{}", welcome);
            println!("{}", usage);
            println!("{table}");
            println!("{}", "Enjoy managing your tasks efficiently with TUSK! 😊".bright_magenta().bold());
        }
    }
    save_tasks_to_file(filename, &accounts)?;
    Ok(())
}

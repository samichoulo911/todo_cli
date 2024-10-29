use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Subcommand)]
pub enum Commands {
    /// List the existing todos
    List,
    /// Add a todo
    Add {
        name: String,
        description: Option<String>,
    },
    /// Remove a todo (permanant action)
    Remove { name: String },
    /// Marks an item as completed
    Complete { name: String },
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct TodoElement {
    completed: bool,
    name: String,
    description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct TodoDataBase {
    elements: Vec<TodoElement>,
}

const FILE_DB: &str = "todo.json";
fn get_db() -> TodoDataBase {
    if let Ok(content) = std::fs::read_to_string(FILE_DB) {
        if !content.is_empty() {
            return serde_json::from_str(&content).unwrap()
        }
    }
    TodoDataBase::default()
}

fn save_db(db: TodoDataBase) {
    let _ = std::fs::write(FILE_DB, serde_json::to_string(&db).unwrap());
}

pub fn add_todo(name: &String, description: &Option<String>) {
    let mut db = get_db();
    db.elements.push(
    TodoElement {
        completed: false,
        name: name.to_string(),
        description: description.clone().unwrap_or("".to_string()),
    });
    save_db(db);
}

pub fn remove_todo(to_remove: &String) {
    let mut db = get_db();
    db.elements = db.elements.into_iter().filter(|e| !e.name.contains(to_remove)).collect();
    save_db(db);
}

pub fn complete_todo(completed: &String) {
    let mut db = get_db();
    for element in db.elements.iter_mut() {
        if element.name.contains(completed) {
            element.completed = true;
        }
    }
    save_db(db);
}

pub fn list_todos() {
    let db = get_db();
    for element in db.elements.iter().filter(|e| !e.completed) {
        if element.description.is_empty() {
            println!("[ ] - {:?}", element.name);
        } else {
            println!("[ ] - {:?}: {:?}", element.name, element.description);
        }
    }
    for element in db.elements.iter().filter(|e| e.completed) {
        if element.description.is_empty() {
            println!("[x] - {:?}", element.name);
        } else {
            println!("[x] - {:?}: {:?}", element.name, element.description);
        }
    }
}

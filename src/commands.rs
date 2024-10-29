use clap::Subcommand;

use crate::database::{self, TodoElement};

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

pub fn add_todo(name: &String, description: &Option<String>) {
    let mut db = database::get_db();
    db.elements.push(
    TodoElement {
        completed: false,
        name: name.to_string(),
        description: description.clone().unwrap_or("".to_string()),
    });
    database::save_db(db);
}

pub fn remove_todo(to_remove: &String) {
    let mut db = database::get_db();
    db.elements.retain(|e| !e.name.contains(to_remove));
    database::save_db(db);
}

pub fn complete_todo(completed: &String) {
    let mut db = database::get_db();
    for element in db.elements.iter_mut() {
        if element.name.contains(completed) {
            element.completed = true;
        }
    }
    database::save_db(db);
}

pub fn list_todos() {
    let db = database::get_db();
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

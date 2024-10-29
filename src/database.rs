use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TodoElement {
    pub completed: bool,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct TodoDataBase {
    pub elements: Vec<TodoElement>,
}

const FILE_DB: &str = "todo.json";
pub fn get_db() -> TodoDataBase {
    if let Ok(content) = std::fs::read_to_string(FILE_DB) {
        if !content.is_empty() {
            return serde_json::from_str(&content).unwrap()
        }
    }
    TodoDataBase::default()
}

pub fn save_db(db: TodoDataBase) {
    let _ = std::fs::write(FILE_DB, serde_json::to_string(&db).unwrap());
}

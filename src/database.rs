use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum DatabaseSource {
    File(String),
    Git(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    source: DatabaseSource,
}

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

fn get_db_config() -> Option<Database> {
    let xdg_directory = xdg::BaseDirectories::with_prefix("todo_cli").unwrap();
    let config_path = xdg_directory
        .place_config_file("database_config.json")
        .expect("Failed to create xdg config file");
    if let Ok(config_content) = std::fs::read_to_string(config_path) {
        return Some(serde_json::from_str(&config_content).unwrap())
    }
    None
}

pub fn get_db() -> TodoDataBase {
    let db_config = get_db_config();
    match db_config.unwrap().source {
        DatabaseSource::File(file_name) => {
            if let Ok(content) = std::fs::read_to_string(file_name) {
                if !content.is_empty() {
                    return serde_json::from_str(&content).unwrap();
                }
            }
        },
        DatabaseSource::Git(_) => todo!()
    }

    TodoDataBase::default()
}

pub fn save_db(db: TodoDataBase) {
    let db_config = get_db_config();
    match db_config.unwrap().source {
        DatabaseSource::File(file_name) => {
            let _ = std::fs::write(file_name, serde_json::to_string(&db).unwrap());
        },
        DatabaseSource::Git(_) => todo!()
    }
}

use auth_git2::GitAuthenticator;
use git2::Repository;
use serde::{Deserialize, Serialize};

static DATABASE_FILE_NAME: &str = "todos.json";
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

fn get_db_from_file(file_path: String) -> Option<TodoDataBase> {
    if let Ok(content) = std::fs::read_to_string(file_path) {
        if !content.is_empty() {
            return Some(serde_json::from_str(&content).unwrap());
        }
    }
    None
}

fn pull_latest_git(git_directory: &str) {
    let repo = Repository::open(&git_directory).unwrap();
    let mut remote = repo.find_remote("origin").unwrap();
    let auth = GitAuthenticator::default();
    auth.fetch(&repo, &mut remote, &["main"], None).unwrap();
    auth.download(&repo, &mut remote, &["main"]).unwrap();
    let fetch_head = repo.find_reference("FETCH_HEAD").unwrap();
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head).unwrap();
    let analysis = repo.merge_analysis(&[&fetch_commit]).unwrap();
    if analysis.0.is_up_to_date() {
        println!("Is up to date, nothing to pull");
    } else if analysis.0.is_fast_forward() {
        let ref_name = "refs/heads/main";
        let mut reference = repo.find_reference(&ref_name).unwrap();
        reference.set_target(fetch_commit.id(), "Fast-Forward").unwrap();
        repo.set_head(&ref_name).unwrap();
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force())).unwrap();
    } else {
        println!("I currently only support fast-forwards");
    }
}

pub fn get_db() -> TodoDataBase {
    let db_config = get_db_config();
    match db_config.unwrap().source {
        DatabaseSource::File(database_path) => {
            let file_path = format!("{}/{}", database_path, DATABASE_FILE_NAME);
            get_db_from_file(file_path).unwrap()
        },
        DatabaseSource::Git(git_path) => {
            pull_latest_git(&git_path);
            let db_file_path = format!("{}/{}", git_path, DATABASE_FILE_NAME);
            get_db_from_file(db_file_path).unwrap()
        }
    }
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

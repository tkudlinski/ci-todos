use druid::{AppLauncher, WindowDesc};
use std::collections::HashMap;

mod data;
use data::AppState;

mod view;
use view::build_ui;

struct Todo {
    map: HashMap<String, bool>,
}

const DB_FILE: &str = "todos.json";
enum Action {
    ADD,
    COMPLETE,
}

impl Action {
    fn as_str(&self) -> &str {
        match self {
            Action::ADD => "add",
            Action::COMPLETE => "complete",
        }
    }
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(DB_FILE)?;
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(DB_FILE)?;
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}

fn main() {
    let action = std::env::args().nth(1).expect("Please provide an action");
    let item = std::env::args().nth(2).expect("Please provide an item");

    let mut todo = Todo::new().expect("Initialisation of db failed");

    if action == Action::ADD.as_str() {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    } else if action == Action::COMPLETE.as_str() {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("Todo saved"),
                Err(why) => println!("An error occurred: {}", why),
            },
        }
    }
}

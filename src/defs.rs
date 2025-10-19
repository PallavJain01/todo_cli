//! Definition and implementation of Todo and TodoFine

use std::{fs::File, io::{BufReader, BufWriter}};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Todo {
	#[serde(rename = "ID")]
	pub id: u16,
	pub title: String,
	pub description: String,
  pub finished: bool
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TodoFile{
	pub todos_count: u16,
	pub todos_per_page: u8,
	pub show_description: bool,
	pub todos: Vec<Todo>
}

impl Todo {
	pub fn new(title: String, description: String, todofile: Option<&TodoFile>) -> Self {
		Todo {
			id: Todo::generate_todo_id(todofile),
			title: title,
			description: description,
      finished: false
		}
	}

	pub fn load(id: u16, todofile: Option<&TodoFile>) -> Todo {
    if let Some(file) = todofile {
      if let Some(todo) = file.todos.iter().find(|t| t.id == id) {
        todo.clone()
      } else {
        Todo::default()
      }
    } else {
      Todo::default()
    }
  }

  pub fn delete(id: u16, todofile: Option<&mut TodoFile>) -> () {
    if let Some(file) = todofile {
      file.todos.retain(|t| t.id != id);
    }
  }

	fn generate_todo_id(todofile: Option<&TodoFile>) -> u16 {
    if let Some(file) = todofile {
      file.todos.last().unwrap().id + 1
    } else {0u16}
	}

  pub fn finish(&mut self, todofile: Option<&mut TodoFile>) {
    self.finished = true;
    if let Some(file) = todofile {
      if let Some(todo) = file.todos.iter_mut().find(|t| t.id == self.id) {
        *todo = self.clone();
      }
    }
  }

}

impl TodoFile {
  pub fn load() -> Self {
    let file = File::open("data.json").expect("Failed to load data.json");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to read json from data.json")
  }
  pub fn add_todo(&mut self, todo: &Todo) {
    self.todos.push(todo.clone());
  }
  pub fn save_to_file(&self) {
    let file = File::create("data.json").expect("Failed to load data.json");
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &self).expect("Failed to write to data.json");
  }
}
//! CLI implementation of todo list

mod cmd;
mod defs;

use clap::Parser;

use crate::{cmd::{Actions, App, Options}, defs::{Todo, TodoFile}};

fn print_todos(page: &u8, todofile: &TodoFile) {
	println!("Current Todos: ({} / {})", page, todofile.todos_count / todofile.todos_per_page as u16 + 1);
	if todofile.show_description == true {
		for todo in &todofile.todos {
			println!("ID: {}\tFinished: {}\nTitle: {}\nDescription: {}", todo.id, todo.finished, todo.title, todo.description);
		}
	} else {
		for todo in &todofile.todos {
			println!("ID: {}\tFinished: {}\nTitle: {}", todo.id, todo.finished, todo.title);
		}
	}
}

fn main() {
	let app = App::parse();

	let mut todofile = TodoFile::load();

	if let Some(page) = app.page {
		print_todos(&page, &todofile);
		return;
	}

	if let Some(action) = &app.actions {
		match action {
			Actions::Add { title, description } => {
				if let Some(t) = title {
					let todo = Todo::new(t.into(), description.into(), Some(&todofile));
					todofile.add_todo(&todo);
					println!("Added todo '{}' with id '{}'", &todo.title, &todo.id);
				} else {
					println!("Title not provided")
				}
			},
			Actions::Finish { id } => {
				let mut todo = Todo::load(*id, Some(&todofile));
				todo.finish(Some(&mut todofile));
				println!("Marked {} as finished", &id);
			},
			Actions::Delete { id } => {
				Todo::delete(*id, Some(&mut todofile));
			},
			Actions::Options { options } => match &options {
				Options::TodosPerPage { n } => {
					todofile.todos_per_page = *n;
					println!("Set todos per page to {}", &n);
				},
				Options::ShowDescription { flag } => {
					todofile.show_description = if todofile.show_description != *flag {*flag} else {!*flag};
					println!("Set show description to {}", &todofile.show_description);
				}
			}
		}
	}
	todofile.save_to_file();
}
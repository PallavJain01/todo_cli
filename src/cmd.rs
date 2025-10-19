//! CLI handler of the todo app

use clap::{
	Parser, Subcommand
};

#[derive(Parser)]
#[command(name="TODO CLI", version="1.0", about="A todo app in the command line", long_about=None)]
pub struct App {
	#[arg(short='l', long="list", help="Todo List", num_args = 0..=1, default_missing_value = "1")]
	pub page: Option<u8>,

	#[command(subcommand)]
	pub actions: Option<Actions>
}

#[derive(Subcommand)]
pub enum Actions {
	Add {
		#[arg(short='t', long="title")]
		title: Option<String>,

		#[arg(short='d', long="description", default_value_t=String::from("No Description"))]
		description: String
	},

	Finish {
		#[arg(short='i', long="id")]
		id: u16
	},

	Delete {
		#[arg(short='i', long="id")]
		id: u16
	},

	Options {
		#[command(subcommand)]
		options: Options
	}
}

#[derive(Subcommand)]
pub enum Options {
	TodosPerPage {
		#[arg(short, long)]
		n: u8
	},

	ShowDescription {
		#[arg(short, long)]
		flag: bool
	}
}
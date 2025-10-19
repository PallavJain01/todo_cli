Todo_CLI

Todo-app in the command line

command:
cargo run --
  add
    -t <title> -d <description>
  finish
    -i <u8>
  delete
    -i <u8>
  list
    -l <u8> (page number. default: 1)
  options
    todos-per-page
      <u8> (default: 8)
    show-description
      (default: false)


todo_cli add <title> <description> => "Added <title> to TODO list. ID: <ID>"
todo_cli finish <ID> => "Finished <title>. ID: <ID>"
todo_cli delete <ID> => "Deleted <title>. ID: <ID>"
todo_cli --list <u8> => "Current TODOs (Page <u8> of <u8>):\n '<ID>: <title>\n\t<description>\n\n'..."
todo_cli options -todos-per-page <u8>
todo_cli options -show-description -f (toggle)
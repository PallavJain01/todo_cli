command:

todo_cli
  --add
    <title> <description>
  --finish
    <ID>
  --delete
    <ID>
  --list
    <u8> (page number. 0 for all todos. default: 1)
  --settings
    --todos-per-page
      <u8> (default: 8)
    --show-description
      <bool> (default: false)


todo_cli add <title> <description> => "Added <title> to TODO list. ID: <ID>"
todo_cli finish <ID> => "Finished <title>. ID: <ID>"
todo_cli delete <ID> => "Deleted <title>. ID: <ID>"
todo_cli --list <u8> => "Current TODOs (Page <u8> of <u8>):\n '<ID>: <title>\n\t<description>\n\n'..."
todo_cli settings


save format:
JSON
{
  todos_count: <u16>,
  todos_per_page: <u8>,
  show_description: <bool>,
  todos: [
    {
      "ID": <u8>
      "title": <String>,
      "description": <String>,
      "finished": <bool>
    }
  ]
}
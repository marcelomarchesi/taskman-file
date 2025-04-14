use std::collections::LinkedList;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::ErrorKind;

use clap::Parser;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    index: u32,
    name: String,
    description: String,
    due_date: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Add task with "name" "description" "due_date": -a "grocery" "buy lemon" "2025-05-05"
    #[arg(long, short = 'a')]
    add_task: Option<String>,

    /// Remove task from list, using its index.
    #[arg(long, short = 'r')]
    remove_task: Option<u32>,

    /// Show all tasks.
    #[arg(long, short = 'l', default_value_t = false)]
    list_task: bool,

    /// Move task to TODO list, using its index.
    #[arg(long, short = 't')]
    task_todo: Option<u32>,

    /// Move task to DOING list, using its index.
    #[arg(long, short = 'd')]
    task_doing: Option<u32>,

    /// Move task to FINISHED list, using its index.
    #[arg(long, short = 'f')]
    task_finished: Option<u32>,
}

fn print_lists(todo: &LinkedList<Task>, doing: &LinkedList<Task>, finished: &LinkedList<Task>) {
    println!("================ TODO LIST ===================");
    for temp_task in todo {
        println!("Index: {}", temp_task.index);
        println!("Name: {}", temp_task.name);
        println!("Task Description: {}", temp_task.description);
        println!("Task Due Date: {}", temp_task.due_date);
        println!("----------------------------------------------");
    }
    println!("==============================================\n");
    println!("================ DOING LIST ==================");
    for temp_task in doing {
        println!("Index: {}", temp_task.index);
        println!("Name: {}", temp_task.name);
        println!("Task Description: {}", temp_task.description);
        println!("Task Due Date: {}", temp_task.due_date);
        println!("----------------------------------------------");
    }
    println!("==============================================\n");
    println!("============== FINISHED LIST =================");
    for temp_task in finished {
        println!("Index: {}", temp_task.index);
        println!("Name: {}", temp_task.name);
        println!("Task Description: {}", temp_task.description);
        println!("Task Due Date: {}", temp_task.due_date);
        println!("----------------------------------------------");
    }
    println!("=============================================\n");
}

fn read_write_example() {
    let file_path: String = String::from("./poem.txt");
    println!("In file {file_path}");
    let mut contents: String = fs::read_to_string(&file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");

    let text_to_write: String = String::from("Marcelo Vieira Marchesi\n");
    println!("Writing to the same file, appending this text: {}", text_to_write);
    contents.push_str(text_to_write.as_str());
    fs::write(&file_path, contents).expect("Should have been able to write to file");
    contents = fs::read_to_string(&file_path)
        .expect("Should have been able to read the file");
    println!("With text append:\n{contents}");
}

fn read_list(file_name: &str) -> String {
    {
        let f: Result<File, std::io::Error> = File::create_new(&file_name);
        match f {
            Err(E) => println!("File already exists {}", file_name),
            Ok(E) => println!("File was created {}", file_name),
        }
    }

    let file_data: String = fs::read_to_string(&file_name).expect("Should have been able to read from file");
    file_data
}

fn recreate_list(file_name: &str) -> LinkedList<Task> {
    let empty_list: LinkedList<Task> = LinkedList::new();
    let file_data: String = read_list(&file_name);
    let task_list: LinkedList<Task> = serde_json::from_str(&file_data).unwrap_or_else(|_| empty_list);
    task_list
}

fn write_list(file_name: &str, file_data: &LinkedList<Task>) {
    let serialized: String = serde_json::to_string(&file_data).unwrap();
    fs::write(&file_name, serialized).expect("Should have been able to write to file");
}

fn main() {
    let args: Args = Args::parse();
    let none_string: String = String::from("none");

    let mut todo_list: LinkedList<Task> = LinkedList::new();
    let mut doing_list: LinkedList<Task> = LinkedList::new();
    let mut finished_list: LinkedList<Task> = LinkedList::new();
    let null_task: Task = Task { index: 255, name: String::from("none"), description: String::from("none"), due_date: String::from("none") };

    // Init all lists by reading the serialized data
    todo_list = recreate_list("todo_list.txt");
    doing_list = recreate_list("doing_list.txt");
    finished_list = recreate_list("finished_list.txt");

    // Change to Vec<String> so it is possible to capture more than the description
    // $ cargo run -- -a "marcelo" "fazer compras do mes"
    let add_t: &String = args.add_task.as_ref().unwrap_or_else(|| &none_string);
    let rem_t: u32 = args.remove_task.unwrap_or_else(|| 0);
    let list_t: bool = args.list_task;
    let todo_t: u32 = args.task_todo.unwrap_or_else(|| 0);
    let doing_t: u32 = args.task_doing.unwrap_or_else(|| 0);
    let finished_t: u32 = args.task_finished.unwrap_or_else(|| 0);

    println!("new add_task {}", add_t);
    println!("new remove_task {}", rem_t);
    println!("new list_t {}", list_t);
    println!("new task_todo {}", todo_t);
    println!("new task_doing {}", doing_t);
    println!("new task_finished {}", finished_t);

    let test_task: Task = Task  {
                                    index: 0,
                                    name: String::from("Teste da Silva"),
                                    description: String::from("Aprender tudo de Rust, Bazel e Python"),
                                    due_date: String::from("2025-04-31"),
                                };
    todo_list.push_back(test_task);
    let new_task: Task = Task   {
                                    index: 0,
                                    name: add_t.clone(),
                                    description: String::from("none"),
                                    due_date: String::from("none"),
                                };
    todo_list.push_back(new_task);
    print_lists(&todo_list, &doing_list, &finished_list);

    // Remove from TODO and put in DOING.
    let mut todo_split: LinkedList<Task> = todo_list.split_off(1);
    doing_list.push_back(todo_split.pop_back().unwrap());
    todo_list.append(&mut todo_split);
    print_lists(&todo_list, &doing_list, &finished_list);

    //read_write_example();

    // Save all lists with new data, either added or removed.
    write_list("todo_list.txt", &todo_list);
    write_list("doing_list.txt", &doing_list);
    write_list("finished_list.txt", &finished_list);

}
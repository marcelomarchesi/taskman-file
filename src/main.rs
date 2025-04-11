use std::collections::LinkedList;
use clap::Parser;

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
    println!("================ DOING LIST ==================");
    for temp_task in doing {
        println!("Index: {}", temp_task.index);
        println!("Name: {}", temp_task.name);
        println!("Task Description: {}", temp_task.description);
        println!("Task Due Date: {}", temp_task.due_date);
        println!("----------------------------------------------");
    }
    println!("============== FINISHED LIST =================");
    for temp_task in finished {
        println!("Index: {}", temp_task.index);
        println!("Name: {}", temp_task.name);
        println!("Task Description: {}", temp_task.description);
        println!("Task Due Date: {}", temp_task.due_date);
        println!("----------------------------------------------");
    }
}

fn main() {
    let args: Args = Args::parse();
    let none_string: String = String::from("none");

    let mut global_index: u32 = 0;
    let mut todo_list: LinkedList<Task> = LinkedList::new();
    let mut doing_list: LinkedList<Task> = LinkedList::new();
    let mut finished_list: LinkedList<Task> = LinkedList::new();
    let null_task: Task = Task { index: 255, name: String::from("NULL"), description: String::from("NULL"), due_date: String::from("NULL") };

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
                                    index: global_index,
                                    name: String::from("Teste da Silva"),
                                    description: String::from("Aprender tudo de Rust, Bazel e Python"),
                                    due_date: String::from("2025-04-31"),
                                };
    todo_list.push_back(test_task);

    global_index+=1;
    let new_task: Task = Task   {
                                    index: global_index,
                                    name: add_t.clone(),
                                    description: String::from("NULL"),
                                    due_date: String::from("NULL"),
                                };
    todo_list.push_back(new_task);

    print_lists(&todo_list, &doing_list, &finished_list);

    // Remove from TODO and put in DOING.
    // If more than 2, would need to rejoin the split and old todo_list with .append, but its not the case here.
    let mut todo_split = todo_list.split_off(1);
    doing_list.push_back(todo_split.pop_back().unwrap());

    print_lists(&todo_list, &doing_list, &finished_list);

}
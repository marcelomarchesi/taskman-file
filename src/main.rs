#![feature(linked_list_remove)]

use std::collections::{LinkedList};
use std::fs;
use std::fs::File;
use core::sync::atomic::{AtomicUsize, Ordering};

use clap::Parser;
use serde::{Serialize, Deserialize};

static GLOBAL_IDX: AtomicUsize = AtomicUsize::new(0);

fn idx_inc() {
    let mut new_idx: usize = idx_get();
    new_idx += 1;
    GLOBAL_IDX.store(new_idx, Ordering::Relaxed);
}

fn idx_get() -> usize {
    GLOBAL_IDX.load(Ordering::Relaxed)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    index: usize,
    name: String,
    description: String,
    due_date: String,
}

impl Task {
    // This will act as a "constructor" for Task structure
    fn new(name: String, description: String, due_date: String) -> Self {
        idx_inc();
        Task {
            index: idx_get(),
            name: name,
            description: description,
            due_date: due_date,
        }
    }

    // Regenerates index for each Task after loading task lists
    fn update_index(&mut self) {
        idx_inc();
        self.index = idx_get();
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Add task to TODO with "name", "description", "due_date": -a "grocery" -a "buy lemon" -a "2025-05-05"
    #[arg(long, short = 'a')]
    add_task: Option<Vec<String>>,

    /// Remove task <idx> from any list.
    #[arg(long, short = 'r')]
    remove_task: Option<usize>,

    /// Show all tasks from all lists.
    #[arg(long, short = 'l', default_value_t = false)]
    list_task: bool,

    /// Move task <idx> from LIST A to LIST B.: -m "3" "doing" "finished"
    #[arg(long, short = 'm')]
    task_move: Option<Vec<String>>,
}

fn print_list(my_list: &LinkedList<Task>, list_name: String) {
    println!("=================== {} LIST =====================", list_name);
    for temp_task in my_list {
        println!("Index: {}", temp_task.index);
        println!("Name: {}", temp_task.name);
        println!("Task Description: {}", temp_task.description);
        println!("Task Due Date: {}", temp_task.due_date);
        println!("----------------------------------------------");
    }
    println!("==================================================\n");
}

fn print_lists(todo: &LinkedList<Task>, doing: &LinkedList<Task>, finished: &LinkedList<Task>) {
    print_list(todo, String::from("TODO"));
    print_list(doing, String::from("DOING"));
    print_list(finished, String::from("FINISHED"));
}

fn read_list(file_name: &str) -> String {
    {
        let f: Result<File, std::io::Error> = File::create_new(&file_name);
        match f {
            Err(_e) => println!("File already exists {}", file_name),
            Ok(_e) => println!("File was created {}", file_name),
        }
    }

    let file_data: String = fs::read_to_string(&file_name).expect("Should have been able to read from file");
    file_data
}

fn recreate_list(file_name: &str) -> LinkedList<Task> {
    let empty_list: LinkedList<Task> = LinkedList::new();
    let file_data: String = read_list(&file_name);
    let mut task_list: LinkedList<Task> = serde_json::from_str(&file_data).unwrap_or_else(|_| empty_list);
    task_list = recreate_index(task_list);
    task_list
}

fn write_list(file_name: &str, file_data: &LinkedList<Task>) {
    let serialized: String = serde_json::to_string(&file_data).unwrap();
    fs::write(&file_name, serialized).expect("Should have been able to write to file");
}

fn recreate_index(mut task_list: LinkedList<Task>) -> LinkedList<Task> {
    for task in task_list.iter_mut() {
        task.update_index();
    }
    task_list
}

fn add_task(args: &Args, todo: &mut LinkedList<Task>) {
    let none_string: String = String::from("none");
    let none_vector: Vec<String> = vec![none_string.clone(), none_string.clone(), none_string.clone()];
    let add_t: &Vec<String> = args.add_task.as_ref().unwrap_or_else(|| &none_vector);

    if add_t.len() == 3 && !args.add_task.is_none() {
        //println!("new add_task name {} desc {} due_date {}", add_t[0], add_t[1], add_t[2]);
        let new_task: Task = Task::new(add_t[0].clone(), add_t[1].clone(), add_t[2].clone());
        todo.push_back(new_task);
        //print_list(&todo, String::from("TODO"));
    } else {
        println!("Not adding a task or missing values!");
    }
}

fn list_tasks(args: &Args, todo: &LinkedList<Task>, doing: &LinkedList<Task>, finished: &LinkedList<Task>) {
    let list_t: bool = args.list_task;
    if list_t {
        print_lists(&todo, &doing, &finished);
    } else {
        println!("Not printing lists of tasks!");
    }
}

fn remove_task(args: &Args, todo: &mut LinkedList<Task>, doing: &mut LinkedList<Task>, finished: &mut LinkedList<Task>) {
    let rem_t: usize = args.remove_task.unwrap_or_else(|| 0);
    if rem_t != 0 {
        if let Some(position) = todo.iter().position(|t| t.index == rem_t) {
            println!("Removing task {} from TODO list", rem_t);
            println!("{:?}", todo.iter().clone().nth(rem_t));
            todo.remove(position);
        }
        if let Some(position) = doing.iter().position(|t| t.index == rem_t) {
            println!("Removing task {} from DOING list", rem_t);
            println!("{:?}", doing.iter().clone().nth(rem_t));
            doing.remove(position);
        }
        if let Some(position) = finished.iter().position(|t| t.index == rem_t) {
            println!("Removing task {} from FINISHED list", rem_t);
            println!("{:?}", finished.iter().clone().nth(rem_t));
            finished.remove(position);
        }
    } else {
        println!("Not deleting any task!");
    }
}

fn move_task(args: &Args, todo: &mut LinkedList<Task>, doing: &mut LinkedList<Task>, finished: &mut LinkedList<Task>) {
    let none_string: String = String::from("none");
    let none_vector: Vec<String> = vec![none_string.clone(), none_string.clone(), none_string.clone()];
    let move_t: &Vec<String> = args.task_move.as_ref().unwrap_or_else(|| &none_vector);

    if move_t.len() == 3 && !args.task_move.is_none() {
        println!("Move some task!");
        let idx: usize = move_t[0].clone().parse().unwrap_or(0);
        let source_list: String = move_t[1].clone();
        let dest_list: String = move_t[2].clone();

        if source_list.contains("todo") && dest_list.contains("doing") {
            let task_to_move = todo.clone().iter().nth(idx).unwrap().clone();
            todo.remove(idx);
            doing.push_back(task_to_move);
        } else if source_list.contains("todo") && dest_list.contains("finished") {
            let task_to_move = todo.clone().iter().nth(idx).unwrap().clone();
            todo.remove(idx);
            finished.push_back(task_to_move);
        } else if source_list.contains("doing") && dest_list.contains("finished") {
            let task_to_move = doing.clone().iter().nth(idx).unwrap().clone();
            doing.remove(idx);
            finished.push_back(task_to_move);
        } else if source_list.contains("doing") && dest_list.contains("todo") {
            let task_to_move = doing.clone().iter().nth(idx).unwrap().clone();
            doing.remove(idx);
            todo.push_back(task_to_move);
        } else if source_list.contains("finished") && dest_list.contains("doing") {
            let task_to_move = finished.clone().iter().nth(idx).unwrap().clone();
            finished.remove(idx);
            doing.push_back(task_to_move);
        } else if source_list.contains("finished") && dest_list.contains("todo") {
            let task_to_move = finished.clone().iter().nth(idx).unwrap().clone();
            finished.remove(idx);
            todo.push_back(task_to_move);
        } else {
            println!("Invalid options to move task {} {} {}", idx, source_list, dest_list);
        }

        /*let mut lists: HashMap<String, &mut LinkedList<Task>> = HashMap::new();
        lists.insert(String::from("todo"), todo);
        lists.insert(String::from("doing"),  doing);
        lists.insert(String::from("finished"),  finished);

        if let Some(source_task) = lists.get(&source_list).unwrap().iter().position(|t| t.index == idx) {
            println!("Moving task {} from {} list", idx, source_list);
            let stask = lists.get(&source_list).unwrap().clone();
            println!("{:?}", *stask.iter().clone().nth(idx-1).unwrap());
            let teste = lists.get(&dest_list).unwrap().clone();
            (*teste).push_back((*stask).iter().clone().nth(idx-1).unwrap().clone());
        }*/
    } else {
        println!("Not moving any task!");
    }
}

fn main() {
    let args: Args = Args::parse();
    let mut todo_list: LinkedList<Task>;
    let mut doing_list: LinkedList<Task>;
    let mut finished_list: LinkedList<Task>;

    // Init all lists by reading the serialized data
    todo_list = recreate_list("todo_list.txt");
    doing_list = recreate_list("doing_list.txt");
    finished_list = recreate_list("finished_list.txt");

    add_task(&args, &mut todo_list);
    list_tasks(&args, &todo_list, &doing_list, &finished_list);
    remove_task(&args, &mut todo_list, &mut doing_list, &mut finished_list);
    move_task(&args, &mut todo_list, &mut doing_list, &mut finished_list);

    // Save all lists with new data, either added or removed.
    write_list("todo_list.txt", &todo_list);
    write_list("doing_list.txt", &doing_list);
    write_list("finished_list.txt", &finished_list);
}
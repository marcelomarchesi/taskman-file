use clap::Parser;

enum TaskList {
    CurrentIndex(u32),
    TodoList(Vec<Task>),
    DoingList(Vec<Task>),
    FinishedList(Vec<Task>),
}

//static NONE_STR: str = "none";
//static NO_STR_VEC: Vec<str> = vec![NONE_STR];

//static CURRENT_INDEX: TaskList = TaskList::CurrentIndex(0);
//static NO_TASK_VEC: Vec<Task> = vec![Task { name: NO_STR_VEC, description: NO_STR_VEC, due_date: NO_STR_VEC }];
//static TODO_LIST: TaskList = TaskList::TodoList(NO_TASK_VEC);
//static DOING_LIST: TaskList = TaskList::DoingList(NO_TASK_VEC);
//static FINISHED_LIST: TaskList = TaskList::FinishedList(NO_TASK_VEC);

struct Task {
    name: String,
    description: String,
    due_date: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Add task with name|description|due_date. Example "grocery|buy lemon|2025-05-05"
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

fn main() {
    let args: Args = Args::parse();
    let none_string: String = String::from("none");

    // Change to Vec<String> so it is possible to capture more than the description
    // $ cargo run -- -a "marcelo" "fazer compras do mes"
    let add_t: &String = args.add_task.as_ref().unwrap_or_else(|| &none_string);
    let rem_t: u32 = args.remove_task.unwrap_or_else(|| 0);
    let list_t: bool = args.list_task;
    let todo_t: u32 = args.task_todo.unwrap_or_else(|| 0);
    let doing_t: u32 = args.task_doing.unwrap_or_else(|| 0);
    let finished_t: u32 = args.task_finished.unwrap_or_else(|| 0);


    //for _ in 0..add_args_len {
        println!("add_task {}", add_t);
        println!("remove_task {}", rem_t);
        println!("list_t {}", list_t);
        println!("task_todo {}", todo_t);
        println!("task_doing {}", doing_t);
        println!("task_finished {}", finished_t);
    //    i+=1;
    //}
}
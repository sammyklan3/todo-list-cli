use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, BufReader, BufWriter};
use std::path::Path;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: String,
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Task {
        Task {
            id: Uuid::new_v4().to_string(),
            description,
            completed: false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    fn load_from_file(file_path: &str) -> io::Result<TodoList> {
        if Path::new(file_path).exists() {
            let file = OpenOptions::new().read(true).open(file_path)?;
            let reader = BufReader::new(file);
            let todo_list: TodoList = serde_json::from_reader(reader)?;
            Ok(todo_list)
        } else {
            Ok(TodoList::new())
        }
    }

    fn save_to_file(&self, file_path: &str) -> io::Result<()> {
        let file = OpenOptions::new().write(true).create(true).truncate(true).open(file_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &self)?;
        Ok(())
    }

    fn add_task(&mut self, description: String) {
        let task = Task::new(description);
        self.tasks.push(task);
    }

    fn remove_task(&mut self, id: &str) {
        self.tasks.retain(|task| task.id != id);
    }

    fn mark_task_completed(&mut self, id: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.completed = true;
        }
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "[{}] {} - {}",
                if task.completed { "x" } else { " " },
                task.id,
                task.description
            );
        }
    }
}

fn main() {
    let file_path = "todo_list.json";
    let mut todo_list = TodoList::load_from_file(file_path).expect("Failed to load to-do list");

    loop {
        println!("1. Add Task");
        println!("2. Remove Task");
        println!("3. Mark Task Completed");
        println!("4. List Tasks");
        println!("5. Save and Exit");
        println!("Choose an option:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let choice = input.trim().parse::<u32>().expect("Please enter a number");

        match choice {
            1 => {
                println!("Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read line");
                let description = description.trim().to_string();
                todo_list.add_task(description);
            }
            2 => {
                println!("Enter task ID to remove:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim();
                todo_list.remove_task(id);
            }
            3 => {
                println!("Enter task ID to mark completed:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim();
                todo_list.mark_task_completed(id);
            }
            4 => {
                todo_list.list_tasks();
            }
            5 => {
                todo_list.save_to_file(file_path).expect("Failed to save to-do list");
                break;
            }
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}

// Define the Task struct
#[derive(Debug, Clone)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

// Create a vector to store instances of the Task struct
struct TaskList {
    tasks: Vec<Task>,
    counter: usize,
}

impl TaskList {
    // Implement the add_task function
    fn add_task(&mut self, description: &str) -> Task {
        let new_task = Task {
            id: self.counter,
            description: String::from(description),
            completed: false,
        };
        self.counter += 1;
        self.tasks.push(new_task.clone());
        new_task
    }

    // Implement the complete_task function
    fn complete_task(&mut self, id: usize) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            Some(task)
        } else {
            None
        }
    }

    // Implement the list_tasks function
    fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "ID: {}, Description: {}, Completed: {}",
                task.id, task.description, task.completed
            );
        }
    }
}

fn main() {
    // Create an instance of the TaskList
    let mut task_list = TaskList {
        tasks: Vec::new(),
        counter: 0,
    };

    // Add tasks
    let task1 = task_list.add_task("Complete Rust homework");
    let task2 = task_list.add_task("Learn Rust basics");

    // List tasks before completion
    println!("Tasks before completion:");
    task_list.list_tasks();

    // Complete a task
    task_list.complete_task(task1.id);

    // List tasks after completion
    println!("\nTasks after completion:");
    task_list.list_tasks();
}

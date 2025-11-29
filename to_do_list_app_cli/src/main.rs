use core::task;
use std::io;
use std::cmp::Ordering;

struct Task {
    task: String,
    status: String,
}


impl Task {
    fn add_task(tasks: &mut Vec<Task>) {
        println!("Please enter the name of the task.");

        let mut task = Task { task: String::new(), status: "new".to_string() };

        io::stdin()
            .read_line(&mut task.task)
            .expect("Failed to readline");

        tasks.push(task);
        println!("New Tasks added!");
    }

    fn remove_task(tasks: &mut Vec<Task>) {
        println!("Please enter the ID of the task you want to delete?");

        let task_len = tasks.len();

        loop {
            let mut idx = String::new();

            io::stdin()
                .read_line(&mut idx)
                .expect("Failed to readline");

            let idx: usize = match idx.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match idx.cmp(&task_len) {
                Ordering::Equal =>  {
                    remove_task(tasks, idx);
                    break;
                },
                Ordering::Less => {
                    remove_task(tasks, idx);
                    break;
                },
                Ordering::Greater => {
                    println!("The id you Entered exceed the number of current tasks {}", task_len);
                 continue;
                },
             }
        }
        fn remove_task(task: &mut Vec<Task>, id: usize) {
            task.remove(id);
            println!("Task removed!");
        }
    }

    fn marked_as_done(tasks: &mut Vec<Task>) {
         println!("Please enter the ID of the task you want to mark as completed?");

        let task_len = tasks.len();

        loop {
            let mut idx = String::new();

            io::stdin()
                .read_line(&mut idx)
                .expect("Failed to readline");

            let idx: usize = match idx.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            match idx.cmp(&task_len) {
                Ordering::Equal =>  {
                    marked_task(tasks, idx);
                    break;
                },
                Ordering::Less => {
                    marked_task(tasks, idx);
                    break;
                },
                Ordering::Greater => {
                    println!("The id you Entered exceed the number of current tasks {}", task_len);
                 continue;
                },
             }
        }

        fn marked_task(tasks: &mut Vec<Task>, id: usize){
            if let Some(task_to_update) = tasks.get_mut(id) {
                task_to_update.status = "Done".to_string();
                println!("Task at index {} is updated to {}", id, task_to_update.status)
            }else {
                println!("Error: error updating task");
            }

        }

    }

}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    println!("Add Task using cli");

    loop {

        if !tasks.is_empty() {
            println!("List of Task");
            println!("__________________________________________");
            println!("|  id  |            tasks                 ");
            println!("__________________________________________");
            for (i, task) in tasks.iter().enumerate() {
                println!("|  {}   |  {} ({})     ", i, task.task.trim(), task.status)
            }
        } else {
            println!("No Task yet");
        }

        println!("Please type action:\n 1 to \"Add\" Task.\n {}0 to \"Exit\"",
             if !tasks.is_empty() {
                "2 to \"Remove\" Task.\n 3 to \"Marked as Done\".\n "
            } else {
                ""
            }
        );

        let mut action = String::new();

        io::stdin()
            .read_line(&mut action)
            .expect("Failed to readline");

        let action: u8 = match action.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        match action {
            1 => Task::add_task(&mut tasks),
            2 => Task::remove_task(&mut tasks),
            3 => Task::marked_as_done(&mut tasks),
            0 => break,
            _ => todo!(),
        }
    }
}



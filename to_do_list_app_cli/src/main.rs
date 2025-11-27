use std::io;
use std::cmp::Ordering;
fn main() {
    let mut tasks: Vec<String> = Vec::new();
    println!("Add Task using cli");

    loop {

        if !tasks.is_empty() {
            println!("List of Task");
            println!("________________________________________");
            println!("|  id  |            tasks               ");
            println!("________________________________________");
            for (i, task) in tasks.iter().enumerate() {
                println!("|  {}   |  {} ", i, task)
            }
        } else {
            println!("No Task yet");
        }

        println!("Please type action:\n 1 to \"Add\" Task.\n {}0 to \"Exit\"",
             if !tasks.is_empty() {
                "2 to \"Remove\" Task.\n "
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
            1 => add_task(&mut tasks),
            2 => remove_task(&mut tasks),
            0 => break,
            _ => todo!(),
        }
    }
}

fn add_task(tasks: &mut Vec<String>) {
    println!("Please enter the name of the task.");

    let mut task = String::new();

    io::stdin()
        .read_line(&mut task)
        .expect("Failed to readline");

    tasks.push(task);
    println!("New Tasks added!");
}

fn remove_task(tasks: &mut Vec<String>) {
   println!("Please enter the ID of the task?");

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
   fn remove_task(task: &mut Vec<String>, id: usize) {
     task.remove(id);
     println!("Task removed!");
   }


}

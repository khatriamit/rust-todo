use std::io;

struct Task {
    description: String,
    completed: bool,
}

fn main() {
    let mut todo_list = Vec::new();

    loop {
        println!("Enter a command ('add', 'mark', 'list') : ");
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();
        if command == "add" {
            println!("Enter a task description: ");

            let mut description = String::new();
            io::stdin().read_line(&mut description).unwrap();
            let description = description.trim();

            todo_list.push(Task {
                description: description.to_owned(),
                completed: false,
            })
        } else if command == "mark" {
            println!("Enter the index of the task to mark it as completed: ");

            let mut index = String::new();
            io::stdin().read_line(&mut index).unwrap();
            let index: usize = index.trim().parse().unwrap();

            if let Some(task) = todo_list.get_mut(index) {
                task.completed = true;
            }
        } else if command == "list" {
            println!("TODO list");

            for (i, task) in todo_list.iter().enumerate() {
                println!(
                    "{} [{}] {}",
                    i,
                    if task.completed { "x" } else { " " },
                    task.description
                );
            }
        } else {
            break;
        }
    }
}

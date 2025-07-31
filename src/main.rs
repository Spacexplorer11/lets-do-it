use indexmap::IndexMap;
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    let mut tasks: IndexMap<String, bool> = IndexMap::new();
    let mut running = true;
    const COMMANDS: [&str; 6] = [
        "add - adds a task",
        "list - lists the tasks",
        "exit - quits the program",
        "help - prints this message",
        "update - updates if a task is done or not!",
        "delete - deletes a task",
    ];

    println!("Welcome to Let's Do It! Please type \"help\" to see all possible commands");

    while running {
        match input(&mut rl).to_lowercase().as_str() {
            "add" => running = add_task(&mut tasks, &mut rl),
            "list" => list_tasks(&tasks),
            "help" => {
                let mut i = 1;
                for command in COMMANDS {
                    println!("{}. {}", i, command);
                    i += 1;
                }
            }
            "delete" => running = delete_task(&mut tasks, &mut rl),
            "" => {}
            "update" => running = update_task(&mut tasks, &mut rl),
            "+!$exit$!+" => {
                println!("Exiting and Saving!");
                break;
            }
            "+!$interrupted$!+" => {
                println!("Interrupted! If you wish to exit, please type \"exit\" or do CTRL+D")
            }
            "+!$error$!+" => println!("Error occurred while taking user input"),
            _ => {
                println!(
                    "Heyo! Thats a command I ain't got in me dictionary, so cant do nothin soz :("
                )
            }
        }
    }
}
fn add_task(tasks: &mut IndexMap<String, bool>, rl: &mut DefaultEditor) -> bool {
    println!("Please enter the task name:");
    let task = input(rl);
    match task.as_str() {
        "+!$exit$!+" => {
            println!("Exiting and Saving!");
            return false;
        }
        "+!$interrupted$!+" => {
            println!("Interrupted! If you wish to exit, please type \"exit\" or do CTRL+D");
            return true;
        }
        "+!$error$!+" => {
            println!("Error occurred while taking user input");
            return true;
        }
        _ => {}
    }
    println!("Task \"{}\" added successfully!", &task);
    tasks.insert(task, false); // false because its not done obvs
    true
}

fn update_task(tasks: &mut IndexMap<String, bool>, rl: &mut DefaultEditor) -> bool {
    if tasks.len() == 0 {
        println!("You have no tasks to update!");
        return true;
    }
    println!("Which task would you like to update? (Pls type the number)");
    list_tasks(&tasks);
    let task = input(rl);
    match task.as_str() {
        "+!$exit$!+" => {
            println!("Exiting and Saving!");
            return false;
        }
        "+!$interrupted$!+" => {
            println!("Interrupted! If you wish to exit, please type \"exit\" or do CTRL+D");
            return true;
        }
        "+!$error$!+" => {
            println!("Error occurred while taking user input");
            return true;
        }
        _ => {}
    }
    if !task.parse::<i32>().is_ok() {
        println!("You didn't enter a number! :( Please try again.");
        return true;
    }
    let task = task.parse::<usize>().unwrap();
    if task > tasks.len() || task < 1 {
        println!("The number you entered is out of range! :( Please try again!");
        return true;
    }
    let task = tasks.get_index(task - 1).map(|(k, _)| k.clone()).unwrap();
    let old_value = tasks.get(&task).unwrap();
    let new_value = !old_value;
    println!(
        "Successfully updated task {}! It's now marked as {}",
        task,
        if new_value { "done" } else { "not done" }
    );
    tasks.insert(task, new_value);
    true
}

fn delete_task(tasks: &mut IndexMap<String, bool>, rl: &mut DefaultEditor) -> bool {
    true
}

fn list_tasks(tasks: &IndexMap<String, bool>) {
    if tasks.len() >= 1 {
        let mut i = 1;
        for (task, &done) in tasks {
            let done_str = if done { "done" } else { "not done" };
            println!("{}. {} - {}", i, task, done_str);
            i += 1;
        }
    } else {
        println!("You have no tasks!")
    }
}

fn input(rl: &mut DefaultEditor) -> String {
    let readline = rl.readline(">> ");
    const EXIT_COMMANDS: [&str; 5] = ["exit", "quit", "q", "logout", "quit"];

    match readline {
        Ok(line) => {
            rl.add_history_entry(line.as_str()).unwrap();
            if EXIT_COMMANDS.contains(&line.trim()) {
                "+!$exit$!+".parse().unwrap()
            } else {
                line.trim().parse().unwrap()
            }
        }

        Err(ReadlineError::Interrupted) => "+!$interrupted$!+".parse().unwrap(),

        Err(ReadlineError::Eof) => "+!$exit$!+".parse().unwrap(),

        Err(err) => {
            eprintln!("Error occurred while taking user input {}", err);
            "+!$error$!+".parse().unwrap()
        }
    }
}

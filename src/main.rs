use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::collections::HashMap;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    let mut tasks: HashMap<String, bool> = HashMap::new();
    const COMMANDS: [&str; 4] = [
        "add - adds a task",
        "list - lists the tasks",
        "exit - quits the program",
        "help - prints this message",
    ];

    println!("Welcome to Let's Do It! Please type \"help\" to see all possible commands");

    loop {
        match input(&mut rl).to_lowercase().as_str() {
            "add" => add_task(&mut tasks, &mut rl),
            "list" => {
                if tasks.len() > 1 {
                    let mut i = 1;
                    for (task, &done) in &tasks {
                        let done_str = if done { "done" } else { "not done" };
                        println!("{}. {} - {}", i, task, done_str);
                        i += 1;
                    }
                } else {
                    println!("You have no tasks!")
                }
            }
            "help" => {
                let mut i = 1;
                for command in COMMANDS {
                    println!("{}. {}", i, command);
                    i += 1;
                }
            }
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
fn add_task(tasks: &mut HashMap<String, bool>, rl: &mut DefaultEditor) {
    println!("Please enter the task name:");
    let task = input(rl);
    println!("Task \"{}\" added successfully!", &task);
    tasks.insert(task, false); // false because its not done obvs
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

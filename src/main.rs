use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    let mut tasks: Vec<String> = Vec::new();

    println!("Welcome to Let's Do It! Please type \"exit\" to exit the program");

    loop {
        match input(&mut rl).to_lowercase().as_str() {
            "add" => add_task(&mut tasks, &mut rl),
            "list" => {
                let mut i = 1;
                for task in &tasks {
                    println!("{}. {}", i, task);
                    i = i + 1;
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
fn add_task(tasks: &mut Vec<String>, rl: &mut DefaultEditor) {
    println!("Please enter the task name:");
    let user_input = input(rl);
    tasks.push(user_input);
}

fn input(rl: &mut DefaultEditor) -> String {
    let readline = rl.readline(">> ");
    const EXIT_COMMANDS: [&str; 4] = ["exit", "quit", "q", "logout"];

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

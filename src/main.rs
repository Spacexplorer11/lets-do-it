use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    let mut tasks: Vec<String> = Vec::new();

    println!("Welcome to Let's Do It! Please type \"exit\" to exit the program");

    loop {
        match input(&mut rl).to_lowercase().as_str() {
            "add" => add_task(&mut tasks, &mut rl),
            "list" => println!("{:?}", tasks),
            "+!$EXIT$!+" => {
                println!("Exiting and Saving!");
                break;
            }
            "+!$INTERRUPTED$!+" => {
                println!("Interrupted! If you wish to exit, please type \"exit\" or do CTRL+D")
            }
            "+!$ERROR$!+" => println!("Error occurred while taking user input"),
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

    match readline {
        Ok(line) => {
            rl.add_history_entry(line.as_str()).unwrap();
            line.trim().parse().unwrap()
        }

        Err(ReadlineError::Interrupted) => "+!$INTERRUPTED$!+".parse().unwrap(),

        Err(ReadlineError::Eof) => "+!$EXIT$!+".parse().unwrap(),

        Err(err) => {
            eprintln!("Error occurred while taking user input {}", err);
            "+!$ERROR$!+".parse().unwrap()
        }
    }
}

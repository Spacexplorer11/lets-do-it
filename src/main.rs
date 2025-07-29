use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();

    println!("Welcome to Let's Do It! Please type \"exit\" to exit the program");

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();

                if !handle_command(&line) {
                    break;
                }
            }

            Err(ReadlineError::Interrupted) => {
                println!("Interrupted. Type 'exit' or press Ctrl-D to quit.");
            }

            Err(ReadlineError::Eof) => {
                println!("Exiting.");
                break;
            }

            Err(err) => {
                println!("An error occurred: {:?}", err);
                break;
            }
        }
    }
}

/// False to exit, true to keep on going
fn handle_command(line: &str) -> bool {
    let parts: Vec<&str> = line.trim().split_whitespace().collect();
    let command = parts.first().unwrap_or(&"");

    match *command {
        "hello" => {
            println!("Hello there!");
        }
        "exit" | "quit" | "logout" | "q" => {
            println!("Exiting & Saving");
            return false;
        }
        "" => {}
        _ => {
            println!("Unknown command: '{}'", line);
        }
    }

    true // Signal to continue the loop.
}

use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

fn main() {
    let mut rl = DefaultEditor::new().unwrap();
    let mut tasks: Vec<String> = Vec::new();

    println!("Welcome to Let's Do It! Please type \"exit\" to exit the program");

    loop {
        match input(&mut rl).as_str() {
            "add" => add_task(),
            _ => {
                println!(
                    "Heyo! Thats a command I ain't got in me dictionary, so cant do nothin soz :("
                )
            }
        }
    }
}
fn add_task() {}

fn input(rl: &mut rustyline::DefaultEditor) -> String {
    let readline = rl.readline(">> ");

    match readline {
        Ok(line) => {
            rl.add_history_entry(line.as_str()).unwrap();

            let parts: Vec<&str> = line.trim().split_whitespace().collect();
            let command = parts.first().unwrap_or(&"");
            line
        }

        Err(ReadlineError::Interrupted) => "+!$INTERRUPTED$!+".parse().unwrap(),

        Err(ReadlineError::Eof) => "+!$EXIT$!+".parse().unwrap(),

        Err(err) => "+!$ERROR$!+".parse().unwrap(),
    }
}

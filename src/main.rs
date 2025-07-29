use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;

fn input() -> Result<(), ReadlineError> {
    let mut rl = DefaultEditor::new()?;

    let readline = rl.readline(">> ");

    match readline {
        Ok(line) => {
            println!("Line: {}", line);
        }
        Err(ReadlineError::Interrupted) => {
            println!("Interrupted (Ctrl-C)");
        }
        Err(ReadlineError::Eof) => {
            println!("End of File (Ctrl-D)");
        }
        Err(err) => {
            println!("An error occurred: {:?}", err);
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = input() {
        eprintln!("Function failed: {:?}", err);
    }
}

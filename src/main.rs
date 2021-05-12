extern crate rustyline;

fn main() {
    let mut done = false;
    let mut reader = rustyline::Editor::<()>::new();
    while !done {
        match reader.readline("> ") {
            Ok(line) =>
                if line.trim() == "(exit)" {
                    done = true;
                } else {
                    println!("{ }", line);
                },
            Err(e) => match e {
                rustyline::error::ReadlineError::Eof => done = true,
                rustyline::error::ReadlineError::Interrupted => done = true,
                _ => println!("Couldn't readline. Error was: {}", e),
            }
        }
    }
}

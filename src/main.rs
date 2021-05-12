extern crate rustyline;
#[macro_use]
extern crate nom;

mod parser;

fn main() {
    let mut reader = rustyline::Editor::<()>::new();
    loop {
        match reader.readline("> ") {
            Ok(line) => {
                if line.trim() == "(exit)" {
                    break;
                } else {
                    println!("{ }", line);
                }
            }
            Err(e) => {
                use rustyline::error::ReadlineError::{Eof, Interrupted};
                match e {
                    Eof | Interrupted => break,
                    _ => println!("Couldn't readline. Error was: {}", e),
                }
            }
        }
    }
}

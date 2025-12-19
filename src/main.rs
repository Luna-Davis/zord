use std::{
    env::{self, args},
    io::{Write, stdin, stdout},
    path::Path,
    process::{Child, Command, Stdio},
};

fn main() {
    loop {
        print!("> ");
        stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split("|").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = input.trim().split_whitespace();
            let command = parts.next().unwrap();
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("~", |x| *x);
                    let home = Path::new(new_dir);
                    if let Err(e) = env::set_current_dir(&home) {
                        eprintln!("{}", e);
                    }
                    previous_command = None;
                }

                "exit" => {
                    println!("Extting!");
                    return;
                }

                command => {
                    let mut stdin = previous_command
                    .map_or(Stdio::inherit(), |output: Child| Stdio::from(output.stdout.unwrap()));
                    let mut child = Command::new(command).args(args).spawn();
                    match child {
                        Ok(mut child) => {
                            child.wait();
                        }

                        Err(e) => {
                            eprintln!("{}", e);
                        }
                    }
                }
            }
        }
    }
}

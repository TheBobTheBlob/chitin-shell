use std::env;
use std::io::{self, Write};

mod commands;

fn main() {
    let mut cmds: Vec<Vec<String>>;
    let mut pipe = String::new();
    let mut cmd_return: Result<String, String>;

    print!("\x1B[2J\x1B[1;1H"); // set output to top of terminal

    println!("chitin shell: version 1.0.0\n");

    'main: loop {
        cmds = input();

        if cmds.is_empty() {
            continue 'main;
        }

        for mut cmd in cmds {
            for part in &mut cmd {
                if part == "%str" {
                    *part = pipe.to_string();
                }
            }

            if cmd.is_empty() {
                continue 'main;
            } else {
                match cmd[0].as_str() {
                    "exit" => break 'main,
                    "folder" => cmd_return = commands::folder::cmd_main(cmd),
                    "file" => cmd_return = commands::file::cmd_main(cmd),
                    "string" => cmd_return = commands::string::cmd_main(cmd),
                    "syscmd" => cmd_return = commands::syscmd::cmd_main(cmd),
                    "math" => cmd_return = commands::math::cmd_main(cmd),
                    "if" => cmd_return = commands::whether::cmd_main(cmd),
                    "help" => cmd_return = commands::help::cmd_main(cmd),
                    _ => {
                        println!("\"{}\" not a recognised command", cmd[0]);
                        continue 'main;
                    }
                }
            }

            pipe = match cmd_return {
                Ok(value) => value,
                Err(value) => {
                    if !value.is_empty() {
                        println!("{}", value);
                    }
                    continue 'main;
                }
            };
        }

        if pipe.trim() != "" {
            println!("{}", pipe.trim());
        }
    }
}

fn input() -> Vec<Vec<String>> {
    let mut user_input = String::new();
    let path = env::current_dir().expect("Cannot access current directory");

    print!("CH {} $ ", path.display());

    io::stdout().flush().unwrap(); // Flushes print buffer as print! does not
    io::stdin()
        .read_line(&mut user_input)
        .expect("Cannot read user inpt");

    user_input = user_input.trim().to_string();
    parser(user_input)
}

fn parser(cmd: String) -> Vec<Vec<String>> {
    let mut cmds = Vec::new();
    let mut single_cmd = Vec::new();
    let mut partial_cmd = String::new();

    let mut previous_backslash = false;
    let mut quotation = false;

    for char in cmd.chars() {
        if !quotation {
            if !previous_backslash {
                match char {
                    '\\' => {
                        previous_backslash = true;
                    }
                    '"' => {
                        quotation = true;
                    }
                    '|' => {
                        cmds.push(single_cmd);
                        single_cmd = Vec::new();
                        partial_cmd = "".to_string();
                    }
                    ' ' => {
                        if !partial_cmd.is_empty() {
                            single_cmd.push(partial_cmd.trim().to_string());
                            partial_cmd = "".to_string();
                        }
                    }
                    _ => {
                        partial_cmd.push(char);
                    }
                }
            } else {
                partial_cmd.push(char);
                previous_backslash = false;
            }
        } else if char == '"' {
            quotation = false;
        } else {
            partial_cmd.push(char);
        }
    }

    if !partial_cmd.is_empty() {
        single_cmd.push(partial_cmd);
    }
    if !single_cmd.is_empty() {
        cmds.push(single_cmd);
    }

    cmds
}

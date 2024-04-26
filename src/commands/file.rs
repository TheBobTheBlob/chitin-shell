use std::fs::File;
use std::io;
use std::io::{Read, Write};

pub fn cmd_main(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() == 1 {
        return Err("\"file\" requires a subcommand. Type \"help file\" for more help".to_string());
    }

    match cmd[1].as_str() {
        "create" => return create(cmd),
        "read" => return read(cmd),
        "write" => return write(cmd),
        "append" => return append(cmd),
        "edit" => return edit(cmd),
        _ => return Err(format!("\"{}\" is not a valid subcommand for file", cmd[1])),
    }
}

fn create(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 3 {
        return Err("\"file create\" requires one parameter".to_string());
    }

    match File::create(&cmd[2]) {
        Ok(_) => return Ok("".to_string()),
        Err(_) => return Err(format!("Could not create file \"{}\"", cmd[2])),
    }
}

fn read(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 3 {
        return Err("\"file read\" requires one parameter".to_string());
    }

    let mut file = match File::open(&cmd[2]) {
        Ok(value) => value,
        Err(_) => return Err(format!("File \"{}\" does not exist", cmd[2])),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    return Ok(contents);
}

fn write(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 4 {
        return Err("\"file write\" requires two parameters".to_string());
    }

    let mut file = match File::options().write(true).truncate(true).open(&cmd[2]) {
        Ok(value) => value,
        Err(_) => return Err(format!("File \"{}\" does not exist", cmd[2])),
    };

    match file.write_all(cmd[3].as_bytes()) {
        Ok(value) => value,
        Err(_) => return Err(format!("File \"{}\" cannot be edited", cmd[2])),
    };

    return Ok("".to_string());
}

fn append(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 4 {
        return Err("\"file append\" requires two parameters".to_string());
    }

    let mut file = match File::options().append(true).open(&cmd[2]) {
        Ok(value) => value,
        Err(_) => return Err(format!("File \"{}\" does not exist", cmd[2])),
    };

    match file.write_all(cmd[3].as_bytes()) {
        Ok(value) => value,
        Err(_) => return Err(format!("File \"{}\" cannot be edited", cmd[2])),
    };

    return Ok("".to_string());
}

fn edit(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 3 {
        return Err("\"file edit\" requires one parameter".to_string());
    }

    let mut file = match File::open(&cmd[2]) {
        Ok(value) => value,
        Err(_) => return Err(format!("File \"{}\" does not exist", cmd[2])),
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut lines = Vec::from_iter(contents.split("\n").map(String::from));
    let mut selected_line = 1;
    let mut print_after;
    let mut print_before;
    let mut user_input;

    loop {
        print!("\x1B[2J\x1B[1;1H"); // set output to top of terminal

        if lines.len() != 0 {
            println!(
                "{}",
                format!(
                    "Editing line {} of {} in file \"{}\"\n",
                    selected_line,
                    lines.len(),
                    cmd[2]
                )
            );

            match lines.len() - selected_line {
                0 => print_before = 6,
                1 => print_before = 5,
                2 => print_before = 4,
                _ => print_before = 3,
            }

            match selected_line - 1 {
                0 => print_after = 6,
                1 => print_after = 5,
                2 => print_after = 4,
                _ => print_after = 3,
            }

            for n in 1..selected_line {
                if selected_line - n <= print_before {
                    println!("  {}", lines[n - 1])
                }
            }

            println!("! {}", lines[selected_line - 1]);

            for n in selected_line..(selected_line + print_after) {
                if n < lines.len() {
                    println!("  {}", lines[n])
                }
            }
        } else {
            println!("File is empty")
        }

        println!("");
        user_input = file_edit_input();

        match user_input.as_str() {
            ":down" | ":d" => {
                if selected_line < lines.len() {
                    selected_line += 1
                }
            }
            ":up" | ":u" => {
                if selected_line > 1 {
                    selected_line -= 1
                }
            }
            ":remove" | ":r" => {
                if lines.len() != 0 {
                    lines.remove(selected_line - 1);
                    if selected_line > lines.len() {
                        selected_line -= 1;
                    }
                }
            }
            ":new" | ":n" => {
                if selected_line != 0 {
                    lines.insert(selected_line - 1, "".to_string())
                } else {
                    lines.push("".to_string());
                    selected_line = 1;
                }
            }
            ":exit" | ":e" => break,
            ":save" | ":s" => {
                let mut file = match File::options().write(true).truncate(true).open(&cmd[2]) {
                    Ok(value) => value,
                    Err(_) => return Err(format!("File \"{}\" does not exist", cmd[2])),
                };

                match file.write_all(lines.join("\n").as_bytes()) {
                    Ok(value) => value,
                    Err(_) => return Err(format!("File \"{}\" cannot be edited", cmd[2])),
                };
            }
            _ => lines[selected_line - 1] = user_input,
        }
    }

    return Ok("".to_string());
}

fn file_edit_input() -> String {
    let mut user_input = String::new();

    print!("> ");

    io::stdout().flush().unwrap(); // Flushes print buffer as print! does not
    io::stdin()
        .read_line(&mut user_input)
        .expect("Cannot read user inpt");

    user_input = user_input.trim().to_string();
    return user_input;
}

pub fn cmd_main(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() == 1 {
        return Err(
            "\"string\" requires a subcommand or parameter. Type \"help string\" for more help"
                .to_string(),
        );
    }

    match cmd[1].as_str() {
        "create" => create(cmd),
        "replace" => replace(cmd),
        "compare" => compare(cmd),
        "includes" => includes(cmd),
        "filter" => filter(cmd),
        _ => Err(format!(
            "\"{}\" is not a valid subcommand for folder",
            cmd[1]
        )),
    }
}

fn create(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 3 {
        return Err("\"string create\" takes a parameters".to_string());
    }

    Ok(cmd[2].to_string())
}

fn replace(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 5 {
        return Err("\"string replace\" takes two parameters".to_string());
    }

    Ok(cmd[2].replace(&cmd[3], &cmd[4]))
}

fn compare(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 4 {
        return Err("\"string compare\" takes two parameters".to_string());
    }

    if cmd[2] == cmd[3] {
        Ok("True".to_string())
    } else {
        Ok("False".to_string())
    }
}

fn includes(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 4 {
        return Err("\"string includes\" takes two parameters".to_string());
    }

    if cmd[2].contains(&cmd[3]) {
        Ok("True".to_string())
    } else {
        Ok("False".to_string())
    }
}

fn filter(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 4 {
        return Err("\"string filter\" takes two parameters".to_string());
    }

    let strings: Vec<&str> = cmd[2]
        .split('\n')
        .filter(|string: &&str| string.contains(&cmd[3]))
        .collect();

    Ok(strings.join("\n"))
}

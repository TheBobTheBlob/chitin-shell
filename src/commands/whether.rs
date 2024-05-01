pub fn cmd_main(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 3 {
        return Err(
            "\"if\" requires a subcommand and a parameter. Type \"help if\" for more help"
                .to_string(),
        );
    }

    match cmd[1].as_str() {
        "true" => return if_true(cmd),
        "false" => return if_false(cmd),
        _ => return Err(format!("\"{}\" is not a valid subcommand for if", cmd[1])),
    }
}

fn if_true(cmd: Vec<String>) -> Result<String, String> {
    if cmd[2] == "True" {
        return Ok("".to_string());
    } else if cmd[2] == "False" {
        return Err("".to_string());
    } else {
        return Err(format!("\"{}\" is not a valid boolean", cmd[1]));
    }
}

fn if_false(cmd: Vec<String>) -> Result<String, String> {
    if cmd[2] == "True" {
        return Err("".to_string());
    } else if cmd[2] == "False" {
        return Ok("".to_string());
    } else {
        return Err(format!("\"{}\" is not a valid boolean", cmd[1]));
    }
}

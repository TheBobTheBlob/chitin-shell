pub fn cmd_main(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 3 {
        return Err(
            "\"if\" requires a subcommand and a parameter. Type \"help if\" for more help"
                .to_string(),
        );
    }

    match cmd[1].as_str() {
        "true" => if_true(cmd),
        "false" => if_false(cmd),
        _ => Err(format!("\"{}\" is not a valid subcommand for if", cmd[1])),
    }
}

fn if_true(cmd: Vec<String>) -> Result<String, String> {
    if cmd[2] == "True" {
        Ok("".to_string())
    } else if cmd[2] == "False" {
        return Err("".to_string());
    } else {
        return Err(format!("\"{}\" is not a valid boolean", cmd[1]));
    }
}

fn if_false(cmd: Vec<String>) -> Result<String, String> {
    if cmd[2] == "True" {
        Err("".to_string())
    } else if cmd[2] == "False" {
        return Ok("".to_string());
    } else {
        return Err(format!("\"{}\" is not a valid boolean", cmd[1]));
    }
}

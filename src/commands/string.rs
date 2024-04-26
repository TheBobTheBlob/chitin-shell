// use regex;

pub fn cmd_main(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() == 1 {
        return Err(
            "\"string\" requires a subcommand or parameter. Type \"help string\" for more help"
                .to_string(),
        );
    }

    match cmd[1].as_str() {
        "create" => return Ok(create(cmd)),
        "replace" => return replace(cmd),
        _ => {
            return Err(format!(
                "\"{}\" is not a valid subcommand for folder",
                cmd[1]
            ))
        }
    }
}

fn create(cmd: Vec<String>) -> String {
    return cmd[2].to_string();
}

fn replace(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 5 {
        return Err("\"string replace\" takes two parameters".to_string());
    }

    return Ok(cmd[2].replace(&cmd[3], &cmd[4]));
}

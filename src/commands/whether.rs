pub fn cmd_main(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 2 {
        return Err("\"if\" requires a parameter. Type \"help if\" for more help".to_string());
    }

    if cmd[1] == "True" {
        return Ok("".to_string());
    } else if cmd[1] == "False" {
        return Err("".to_string());
    } else {
        return Err(format!("\"{}\" is not a valid boolean", cmd[1]));
    }
}

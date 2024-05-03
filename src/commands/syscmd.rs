pub fn cmd_main(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() < 2 {
        return Err(
            "\"syscmd\" needs at least one parameter. Type \"help syscmd\" for more help"
                .to_string(),
        );
    }

    let external_call: Result<std::process::Output, std::io::Error>;
    if cfg!(windows) {
        external_call = std::process::Command::new("powershell")
            .args(&cmd[1..])
            .output();
    } else if cmd.len() > 3 {
        external_call = std::process::Command::new(&cmd[1]).args(&cmd[2..]).output();
    } else {
        external_call = std::process::Command::new(&cmd[1]).output();
    }

    let cmd_result = match external_call {
        Ok(value) => value,
        Err(_) => {
            return Err(format!("\"{}\" is not a recognised system command", cmd[1]));
        }
    };

    let test = String::from_utf8(cmd_result.stdout).unwrap();

    Ok(test)
}

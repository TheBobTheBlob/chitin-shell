pub fn cmd_main(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() < 2 {
        return Err(
            "\"syscmd\" needs at least one parameter. Type \"help syscmd\" for more help"
                .to_string(),
        );
    }

    let external_cmd: Result<std::process::Output, std::io::Error>;
    let cmd_return: String;

    if cfg!(windows) {
        external_cmd = std::process::Command::new("powershell")
            .args(&cmd[1..])
            .output();
    } else if cmd.len() > 3 {
        external_cmd = std::process::Command::new(&cmd[1]).args(&cmd[2..]).output();
    } else {
        external_cmd = std::process::Command::new(&cmd[1]).output();
    }

    let external_return = match external_cmd {
        Ok(value) => value,
        Err(_) => {
            return Err(format!("\"{}\" is not a recognised system command", cmd[1]));
        }
    };

    if external_return.status.success() {
        cmd_return = String::from_utf8(external_return.stdout).unwrap();
    } else {
        cmd_return = String::from_utf8(external_return.stderr).unwrap();
    }

    Ok(cmd_return)
}

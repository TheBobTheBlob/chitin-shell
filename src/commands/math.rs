pub fn cmd_main(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() == 1 {
        return Err("\"math\" requires a subcommand. Type \"help math\" for more help".to_string());
    }

    match cmd[1].as_str() {
        "add" | "subtract" | "multiply" | "divide" => return elementary(cmd),
        "sin" | "cos" | "tan" => return trig(cmd),
        _ => return Err(format!("\"{}\" is not a valid subcommand for math", cmd[1])),
    }
}

fn str_to_float(num: &String) -> Result<f64, String> {
    match num.parse::<f64>() {
        Ok(value) => return Ok(value),
        Err(_) => return Err(format!("\"{}\" is not a valid number", num)),
    };
}

fn elementary(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 4 {
        return Err(format!("\"math {}\" requires two parameters", cmd[1]));
    }

    let num1 = str_to_float(&cmd[2])?;

    let num2 = str_to_float(&cmd[3])?;

    if cmd[1] == "divide" && cmd[3] == "0" {
        return Err("The denominator cannot be zero".to_string());
    }

    match cmd[1].as_str() {
        "add" => return Ok((num1 + num2).to_string()),
        "subtract" => return Ok((num1 + num2).to_string()),
        "multiply" => return Ok((num1 + num2).to_string()),
        "divide" => return Ok((num1 + num2).to_string()),
        _ => unreachable!(),
    }
}

fn trig(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 4 {
        return Err(format!("\"math {}\" requires two parameters", cmd[1]));
    }

    let num = str_to_float(&cmd[2])?;

    match cmd[3].as_str() {
        "degree" => {
            let radians = (num * std::f64::consts::PI) / 180.0;
            match cmd[1].as_str() {
                "sin" => return Ok(radians.sin().to_string()),
                "cos" => return Ok(radians.cos().to_string()),
                "tan" => return Ok(radians.tan().to_string()),
                _ => unreachable!(),
            }
        }
        "radian" => match cmd[1].as_str() {
            "sin" => return Ok(num.sin().to_string()),
            "cos" => return Ok(num.cos().to_string()),
            "tan" => return Ok(num.tan().to_string()),
            _ => unreachable!(),
        },
        _ => {
            return Err(format!(
                "\"math {}\" only supports \"degree\" and \"radian\"",
                cmd[1]
            ))
        }
    }
}

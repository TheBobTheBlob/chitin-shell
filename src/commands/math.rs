pub fn cmd_main(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() == 1 {
        return Err("\"math\" requires a subcommand. Type \"help math\" for more help".to_string());
    }

    match cmd[1].as_str() {
        "add" | "subtract" | "multiply" | "divide" => elementary(cmd),
        "sin" | "cos" | "tan" => trig(cmd),
        "greater" | "less" | "equal" => compare(cmd),
        _ => Err(format!("\"{}\" is not a valid subcommand for math", cmd[1])),
    }
}

fn str_to_float(num: &String) -> Result<f64, String> {
    match num.parse::<f64>() {
        Ok(value) => Ok(value),
        Err(_) => Err(format!("\"{}\" is not a valid number", num)),
    }
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
        "add" => Ok((num1 + num2).to_string()),
        "subtract" => Ok((num1 + num2).to_string()),
        "multiply" => Ok((num1 + num2).to_string()),
        "divide" => Ok((num1 + num2).to_string()),
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
                "sin" => Ok(radians.sin().to_string()),
                "cos" => Ok(radians.cos().to_string()),
                "tan" => Ok(radians.tan().to_string()),
                _ => unreachable!(),
            }
        }
        "radian" => match cmd[1].as_str() {
            "sin" => Ok(num.sin().to_string()),
            "cos" => Ok(num.cos().to_string()),
            "tan" => Ok(num.tan().to_string()),
            _ => unreachable!(),
        },
        _ => Err(format!(
            "\"math {}\" only supports \"degree\" and \"radian\"",
            cmd[1]
        )),
    }
}

fn compare(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 4 {
        return Err(format!("\"math {}\" requires two parameters", cmd[1]));
    }

    let num1 = str_to_float(&cmd[2])?;
    let num2 = str_to_float(&cmd[3])?;

    match cmd[1].as_str() {
        "greater" => {
            if num1 > num2 {
                Ok("True".to_string())
            } else {
                Ok("False".to_string())
            }
        }
        "less" => {
            if num1 < num2 {
                Ok("True".to_string())
            } else {
                Ok("False".to_string())
            }
        }
        "equal" => {
            if num1 == num2 {
                Ok("True".to_string())
            } else {
                Ok("False".to_string())
            }
        }
        _ => unreachable!(),
    }
}

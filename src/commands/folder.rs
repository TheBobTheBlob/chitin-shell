use chrono;
use std::env;
use std::fs;
use std::path::PathBuf;

pub fn cmd_main(cmd: Vec<String>) -> Result<String, String> {
    let cwd = match std::env::current_dir() {
        Ok(value) => value,
        Err(_) => return Err("Cannot access current working folder".to_string()),
    };

    if cmd.len() == 1 {
        return Err(
            "\"folder\" requires a subcommand. Type \"help folder\" for more help".to_string(),
        );
    }

    match cmd[1].as_str() {
        "up" => up(cwd, cmd),
        "list" => list(cwd),
        "down" => down(cwd, cmd),
        "create" => create(cmd),
        "rename" => rename(cmd),
        "delete" => delete(cmd),
        _ => Err(format!(
            "\"{}\" is not a valid subcommand for folder",
            cmd[1]
        )),
    }
}

fn folder_contents(cwd: &PathBuf) -> Result<std::fs::ReadDir, String> {
    let paths_result = fs::read_dir(cwd);

    match paths_result {
        Ok(value) => Ok(value),
        Err(value) => Err(format!("Cannot access current folder: {}", value)),
    }
}

fn up(cwd: PathBuf, cmd: Vec<String>) -> Result<String, String> {
    let parent_dir;

    if cmd.len() == 3 {
        parent_dir = match cwd.display().to_string().split_once(&cmd[2]) {
            Some(value) => PathBuf::from(format!("{}/{}", value.0, cmd[2])),
            None => cwd,
        }
    } else if cmd.len() == 2 {
        parent_dir = match cwd.parent() {
            None => cwd,
            Some(value) => PathBuf::from(value),
        };
    } else {
        return Err("\"folder up\" only takes one optional parameter".to_string());
    }

    match env::set_current_dir(parent_dir) {
        Ok(_) => Ok("".to_string()),
        Err(_) => Err("Cannot access parent folder".to_string()),
    }
}

fn list(cwd: PathBuf) -> Result<String, String> {
    let paths = folder_contents(&cwd)?;
    let mut folder_contents = String::from("TYPE      SIZE  READONLY  LAST MODIFIED        NAME\n");

    for path_result in paths {
        let path = match path_result {
            Ok(value) => value.path(),
            Err(value) => return Err(format!("Cannot access current folder: {value}")),
        };

        // File size

        let mut file_size = path.metadata().unwrap().len();
        let size_extensions = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB"];

        let mut loop_runs = 0;
        let mut size;

        loop {
            if file_size == 0 {
                size = "     ".to_string();
                break;
            } else if file_size > 1024 {
                file_size /= 1024;
                loop_runs += 1;
            } else {
                let extension = size_extensions[loop_runs];
                if file_size < 10 {
                    size = format!("  {file_size}{extension}");
                } else if file_size < 100 {
                    size = format!(" {file_size}{extension}");
                } else {
                    size = format!("{file_size}{extension}");
                }

                if extension == "B" {
                    size = format!(" {size}");
                }
                break;
            }
        }

        // Readonly

        let readonly = if path.metadata().unwrap().permissions().readonly() {
            "True ".to_string()
        } else {
            "False".to_string()
        };

        // Modified

        let datetime: chrono::DateTime<chrono::Utc> =
            path.metadata().unwrap().modified().unwrap().into();

        // Print out

        if path.is_file() {
            folder_contents += format!(
                "File     {size}  {readonly}     {}  {}\n",
                datetime.format("%Y-%m-%d %T"),
                path.file_name().unwrap().to_str().unwrap()
            )
            .as_str()
        } else {
            folder_contents += format!(
                "Folder   {size}  {readonly}     {}  {}\n",
                datetime.format("%Y-%m-%d %T"),
                path.file_name().unwrap().to_str().unwrap()
            )
            .as_str()
        }
    }

    Ok(folder_contents)
}

fn down(cwd: PathBuf, cmd: Vec<String>) -> Result<String, String> {
    let mut paths = folder_contents(&cwd)?;
    let path_num = folder_contents(&cwd)?.count();

    if path_num == 0 {
        return Err("The current folder has no subfolders".to_string());
    }

    if cmd.len() == 2 {
        if path_num == 1 {
            let child_pathbuf = paths.next().unwrap().unwrap().path();

            let child = match child_pathbuf.to_str() {
                Some(value) => value,
                None => return Err("Cannot access current folder".to_string()),
            };

            let _ = env::set_current_dir(child);
            return Ok(env::current_dir().unwrap().to_str().unwrap().to_string());
        } else {
            Err("There is more than one subfolder, please specifiy".to_string())
        }
    } else if cmd.len() == 3 {
        match env::set_current_dir(format!("{}/{}", cwd.to_str().unwrap(), cmd[2])) {
            Ok(_) => Ok("".to_string()),
            Err(_) => Err(format!(
                "\"{}\" is not a subfolder of the current folder",
                cmd[2]
            )),
        }
    } else {
        Err("\"folder down\" takes one parameter".to_string())
    }
}

fn create(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 3 {
        return Err("\"folder create\" requires one parameter".to_string());
    }

    match fs::create_dir_all(&cmd[2]) {
        Ok(_) => Ok("".to_string()),
        Err(_) => Err(format!("Cannot create folder \"{}\"", cmd[2])),
    }
}

fn rename(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 4 {
        return Err("\"folder rename\" requires two parameters".to_string());
    }

    match fs::rename(&cmd[2], &cmd[3]) {
        Ok(_) => Ok("".to_string()),
        Err(_) => Err(format!("Could not rename folder \"{}\"", cmd[2])),
    }
}

fn delete(cmd: Vec<String>) -> Result<String, String> {
    if cmd.len() != 3 {
        return Err("\"folder delete\" requires one parameter".to_string());
    }

    match fs::remove_dir_all(&cmd[2]) {
        Ok(_) => Ok("".to_string()),
        Err(_) => Err(format!("Cannot delete folder \"{}\"", cmd[2])),
    }
}

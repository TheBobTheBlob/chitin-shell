pub fn cmd_main(cmd: Vec<String>) -> Result<String, String> {
    const HELP_BASE: &str = "help: get in-shell help on commands\n\n\
                            Mandatory arguments are shown using square brackets []\n\
                            Optional parameters by curly brackets {}\n\n\
                            Commands:\n\
                            help {command} {subcommands}";

    if cmd.len() == 1 {
        return Ok(HELP_BASE.to_string());
    }

    match cmd[1].as_str() {
        "file" => return file(cmd),
        "folder" => return folder(cmd),
        "help" => return help(),
        "math" => return math(cmd),
        "string" => return string(cmd),
        "syscmd" => return syscmd(),
        "if" => return whether(cmd),
        "exit" => return exit(),
        _ => return Err(format!("\"{}\" is not a recognised command", cmd[1])),
    }
}

fn file(cmd: Vec<String>) -> Result<String, String> {
    const FILE_BASE: &str = "file: manipulate files\n\n\
                            Commands:\n\
                            file create [file_name]\n\
                            file read [file_name]\n\
                            file write [file_name] [text]\n\
                            file append [file_name] [text]\n\
                            file edit [file_name]";

    const FILE_CREATE: &str = "file create [file_name]\n\n\
                            Create a file with the given file name";

    const FILE_RENAME: &str = "file rename [old_name] [new_name]\n\n\
                            Rename the file old_name into new_name";

    const FILE_DELETE: &str = "file delete [file_name]\n\n\
                            Delete the file with the given file name";

    const FILE_READ: &str = "file read [file_name]\n\n\
                            Read the contents of a file";

    const FILE_WRITE: &str = "file write [file_name]\n\n\
                            Completely replace the contents of a file";

    const FILE_APPEND: &str = "file append [file_name]\n\n\
                            Append more text to the end of a file";

    const FILE_EDIT: &str = "file edit [file_name]\n\n\
                            Live edit a file in the shell\n\n\
                            Running the command above will show some information about the file, \
                            a preview of lines around the selected line, and an input. The currently \
                            selected line is shown using a exclamation mark `!`, \
                            and the user input is denoted using an angle bracket `>`.\n\n\
                            Entering text and pressing enter will replace the selected line with the text. \
                            There are also some commands that can be run, which are prefaced with a colon `:`.\n\n\
                            - `:down` or `:d` - Select the line below the current\n\
                            - `:up` or `:u` - Select the line above the current\n\
                            - `:remove` or `:r` - Remove the selected line\n\
                            - `:new` or `:n` - Add a new blank line below the current selected line\n\
                            - `:save` or `:s` - Save the changes to the file to disk\n\
                            - `:exit` or `:e` - Exit the file editor";

    if cmd.len() == 2 {
        return Ok(FILE_BASE.to_string());
    }

    match cmd[2].as_str() {
        "create" => return Ok(FILE_CREATE.to_string()),
        "rename" => return Ok(FILE_RENAME.to_string()),
        "delete" => return Ok(FILE_DELETE.to_string()),
        "read" => return Ok(FILE_READ.to_string()),
        "write" => return Ok(FILE_WRITE.to_string()),
        "append" => return Ok(FILE_APPEND.to_string()),
        "edit" => return Ok(FILE_EDIT.to_string()),
        _ => return Err(format!("\"{}\" is not a subcommand for file", cmd[2])),
    }
}

fn folder(cmd: Vec<String>) -> Result<String, String> {
    const FOLDER_BASE: &str = "folder: navigate the file system\n\n\
                            Commands:\n\
                            folder up {folder_name}\n\
                            folder down {folder_name}\n\
                            folder list\n\";
                            folder create [folder_name]\n\";
                            folder rename [old_name] [new_name]\n\";
                            folder delete [folder_name]";

    const FOLDER_UP: &str = "folder up\n\n\
                            Move to the parent folder. If folder_name is given, \
                            the command will move to the highest level with that name.";

    const FOLDER_DOWN: &str = "folder down {folder_name}\n\n\
                            Move to a child folder. If no folder is given and there is \
                            only one child folder, that will automatically be selected.";

    const FOLDER_LIST: &str = "folder list\n\n\
                            List the contents of the current folder";

    const FOLDER_CREATE: &str = "folder create [folder_name]\n\n\
                            Create an empty folder";

    const FOLDER_RENAME: &str = "folder rename [old_name] [new_name]\n\n\
                                Rename the folder old_name into new_name";

    const FOLDER_DELETE: &str = "folder delete [folder_name]\n\n\
                            Delete a folder and all its contents";

    if cmd.len() == 2 {
        return Ok(FOLDER_BASE.to_string());
    }

    match cmd[2].as_str() {
        "up" => return Ok(FOLDER_UP.to_string()),
        "down" => return Ok(FOLDER_DOWN.to_string()),
        "list" => return Ok(FOLDER_LIST.to_string()),
        "create" => return Ok(FOLDER_CREATE.to_string()),
        "rename" => return Ok(FOLDER_RENAME.to_string()),
        "delete" => return Ok(FOLDER_DELETE.to_string()),
        _ => return Err(format!("\"{}\" is not a subcommand for folder", cmd[2])),
    }
}

fn help() -> Result<String, String> {
    const HELP_BASE: &str = "help: in-shell help\n\n\
                            Commands:\n\
                            help {cmd} {subcmd}";

    return Ok(HELP_BASE.to_string());
}

fn math(cmd: Vec<String>) -> Result<String, String> {
    const MATH_BASE: &str = "math: basic mathematical operations\n\n\
                            Commands:\n\
                            math add [num1] [num2]\n\
                            math subtract [num1] [num2]\n\
                            math multiply [num1] [num2]\n\
                            math divide [num1] [num2]\n\
                            math sin [number] [type]\n\
                            math cos [number] [type]\n\
                            math tan [number] [type]\n\
                            math greater [num1] [num2]\n\
                            math less [num1] [num2]\n\
                            math equal [num1] [num2]";

    const MATH_ADD: &str = "math add [num1] [num2]\n\n\
                            Adds the two numbers together";

    const MATH_SUBTRACT: &str = "math subtract [num1] [num2]\n\n\
                            Subtracts the second number from the first";

    const MATH_MULTIPLY: &str = "math multiply [num1] [num2]\n\n\
                            Multiplies the two numbers together";

    const MATH_DIVIDE: &str = "math divide [num1] [num2]\n\n\
                            Divides the first number by the second";

    const MATH_SIN: &str = "math sin [num1] [type]\n\n\
                            Find the sine of a value. \
                            The type can either be `degree` or `radian`.";

    const MATH_COS: &str = "math cos [num1] [type]\n\n\
                            Find the cosine of a value. \
                            The type can either be `degree` or `radian`.";

    const MATH_TAN: &str = "math tan [num1] [type]\n\n\
                            Find the tangent of a value. \
                            The type can either be `degree` or `radian`.";

    const MATH_GREATER: &str = "math greater [num1] [num2]\n\n\
                            Returns \"True\" if num1 is greater than num2, or \"False\" otherwise";

    const MATH_LESS: &str = "math less [num1] [num2]\n\n\
                            Returns \"True\" if num1 is less than num2, or \"False\" otherwise";

    const MATH_EQUAL: &str = "math equal [num1] [num2]\n\n\
                            Returns \"True\" if num1 is equal to num2, or \"False\" otherwise";

    if cmd.len() == 2 {
        return Ok(MATH_BASE.to_string());
    }

    match cmd[2].as_str() {
        "add" => return Ok(MATH_ADD.to_string()),
        "subtract" => return Ok(MATH_SUBTRACT.to_string()),
        "multiply" => return Ok(MATH_MULTIPLY.to_string()),
        "divide" => return Ok(MATH_DIVIDE.to_string()),
        "sin" => return Ok(MATH_SIN.to_string()),
        "cos" => return Ok(MATH_COS.to_string()),
        "tan" => return Ok(MATH_TAN.to_string()),
        "greater" => return Ok(MATH_GREATER.to_string()),
        "less" => return Ok(MATH_LESS.to_string()),
        "equal" => return Ok(MATH_EQUAL.to_string()),
        _ => return Err(format!("\"{}\" is not a subcommand for math", cmd[2])),
    }
}

fn string(cmd: Vec<String>) -> Result<String, String> {
    const STRING_BASE: &str = "string: Manipulate strings\n\n\
                            Commands:\n\
                            string create [text]\n\
                            string replace [text] [string_to_replace] [replacement_text]\n\";
                            string compare [text] [string_to_replace] [replacement_text]\n\";
                            string includes [text1] [text2]\n\";
                            string filter [multiline] [text]";

    const STRING_CREATE: &str = "string create [text]\n\n\
                            Enter a string into the shell to use with piping";

    const STRING_REPLACE: &str = "string replace [text] [string_to_replace] [replacement_text]\n\n\
                            Replace all occurances of a string in some text";

    const STRING_COMPARE: &str = "string compare [text1] [text2]\n\n\
                            Returns \"True\" if text1 and text2 are the same, or \"False\" otherwise";

    const STRING_INCLUDES: &str = "string includes [text1] [text2]\n\n\
                            Returns \"True\" if text1 contains text2, or \"False\" otherwise";

    const STRING_FILTER: &str = "string filter [multiline] [text]\n\n\
                            Filters every line in a multiline string depending on if they contain text";

    if cmd.len() == 2 {
        return Ok(STRING_BASE.to_string());
    }

    match cmd[2].as_str() {
        "create" => return Ok(STRING_CREATE.to_string()),
        "replace" => return Ok(STRING_REPLACE.to_string()),
        "compare" => return Ok(STRING_COMPARE.to_string()),
        "includes" => return Ok(STRING_INCLUDES.to_string()),
        "filter" => return Ok(STRING_FILTER.to_string()),
        _ => return Err(format!("\"{}\" is not a subcommand for string", cmd[2])),
    }
}

fn syscmd() -> Result<String, String> {
    const SYSCMD_BASE: &str = "syscmd: Send a command to the shell of the operating system\n\n\
                            Some commands, like switching directories, may not work.\n\n\
                            Commands:\n\
                            syscmd [cmd] {args1} {arg2} ...";

    return Ok(SYSCMD_BASE.to_string());
}

fn whether(cmd: Vec<String>) -> Result<String, String> {
    const IF_BASE: &str = "if: Continue or stop a command chain\n\n\
                            Will decide on whether to continue the current command chain based on the boolean value\n\n\
                            Commands:\n\
                            if true [boolean]\n\
                            if false [boolean]";

    const IF_TRUE: &str = "if true [boolean]\n\n\
                            Continues the command chain if the boolean is equal to \"True\"";

    const IF_FALSE: &str = "if false [boolean]\n\n\
                            Continues the command chain if the boolean is equal to \"False\"";

    if cmd.len() == 2 {
        return Ok(IF_BASE.to_string());
    }

    match cmd[2].as_str() {
        "true" => return Ok(IF_TRUE.to_string()),
        "false" => return Ok(IF_FALSE.to_string()),
        _ => return Err(format!("\"{}\" is not a subcommand for if", cmd[2])),
    }
}

fn exit() -> Result<String, String> {
    const EXIT_BASE: &str = "exit: exit the shell\n\n\
                            Commands:\n\
                            exit";

    return Ok(EXIT_BASE.to_string());
}

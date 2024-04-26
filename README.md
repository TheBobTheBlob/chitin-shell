# chitin

`chitin` is a simple shell written in Rust for a university honours project, as a way to start learning Rust.

## Building

All building is done using `cargo`. Two external crates are required, `chrono` and `regex`. After cloning the repository, in the directory run one of the following commands:

```shell
cargo build
```

```shell
cargo build --release
```

The shell can then be accessed by running the executable or by the `cargo run` command.

## Shell Basics

After running the shell, the shell will output the version, then the user input field. This field will be of the form

```shell
CH path/to/cwd/ $
```

### Entering Commands

Commands and their parameters are split up by spaces. To make sure a parameter with a space is created as one instead of two, wrap it in double quotes `""`. All characters within double quotes are directly passed on.

```shell
string create "Hello World"
```

The first character after a backwards slash `\` is also directly passed forward.

### Piping

Multiple commands can be entered at once by splitting them up with the pipe `|` character. Commands are run sequentially, from left to right. To pass the output of one command as a parameter to the proceeding command, type `%str` where that parameter would normally go.

```shell
file read text.txt | string replace %str "1.0.0" "2.0.0"
```

## Commands

- `file`: file manipulation
- `folder`: folder traversal
- `help`: in-shell help
- `math`: basic arthimethic and trignometry
- `string`: string manipulation
- `syscmd`: base OS shell link

Mandatory parameters are surrounded by square brackets `[]` and optional parameters are surrounded by curly brackets `{}`. These brackets should not be part of the command when using the shell.

### File

Create a file with the given file name

```shell
file create [file_name]
```

Read the contents of a file

```shell
file read [file_name]
```

Completely replace the contents of a file

```shell
file write [file_name] [text]
```

Append more text to the end of a file

```shell
file append [file_name] [text]
```

Live edit a file in the shell

```shell
file edit [file_name]
```

Running the command above will show some information about the file, a preview of lines around the selected line, and an input. The currently selected line is showen using a exclamation mark `!`, and the user input is denoted using an angle bracket `>`.

```shell
Editing line 1 of 3 in file "file.txt"

! This is line 1
  Another line in the file
  A third line

>
```

Entering text and pressing enter will replace the selected line with the text. There are also some commands that can be run, which are prefaced with a colon `:`.

- `:down` or `:d` - Select the line below the current
- `:up` or `:u` - Select the line above the current
- `:remove` or `:r` - Remove the selected line
- `:new` or `:n` - Add a new blank line below the current selected line
- `:save` or `:s` - Save the changes to the file to disk
- `:exit` or `:e` - Exit the file editor

### Folders

Move to the parent folder

```shell
folder up
```

Move to a child folder. If no folder is given and there is only one child folder, that will automatically be selected.

```shell
folder down {folder_name}
```

List the contents of the current folder

```shell
folder list
```

### Math

Add, subtract, multiply, or divide two numbers.

```shell
math add [num1] [num2]
math subtract [num1] [num2]
math multiply [num1] [num2]
math divide [num1] [num2]
```

Find the sine, cosine, or tangent of a value. The type can either be `degree` or `radian`.

```shell
math sin [number] [type]
math cos [number] [type]
math tan [number] [type]
```

### String

Enter a string into the shell to use with piping

```shell
string create [text]
```

Replace all occurances of a string in some text

```shell
string replace [text] [string_to_replace] [replacement_text]
```

### Syscmd

Send a command to the shell of the operating system. Some commands, like switching directories, may not work.

```shell
syscmd [cmd] {args1} {arg2} ...
```

The arguments of the command should be given as separate parameters, and not as a single string. For example, `"ls -la"` will try to run a command named "la -la".

# chitin

`chitin` is a simple shell written in Rust for a university honours project, as a way to start learning Rust.

## Building

All building is done using `cargo`. Two external crates are required, `chrono` and `regex`. After cloning the repository, in the directory run one of the following commands:

```console
cargo build
```

```console
cargo build --release
```

The shell can then be accessed by running the executable or by the `cargo run` command.

## Shell Basics

After running the shell, the shell will output the version, then the user input field. This field will be of the form

```console
CH path/to/cwd/ $
```

### Entering Commands

Commands and their parameters are split up by spaces. To make sure a parameter with a space is created as one instead of two, wrap it in double quotes `""`. All characters within double quotes are directly passed on.

```console
string create "Hello World"
```

The first character after a backwards slash `\` is also directly passed forward.

### Piping

Multiple commands can be entered at once by splitting them up with the pipe `|` character. Commands are run sequentially, from left to right.

Outputs are automatically saved, and can be accessed in following commands by `%[num]`, where `[num]` is an integer. The number starts at 1 and counts up for every output-producing command. Commands that don't have outputs do not affect this number. To pass the output of the previous command, use `%str`.

```console
math add 5 7 | file create "math.txt" | math subtract 19 7  | math equal %1 %2
file read text.txt | string replace %str "1.0.0" "2.0.0"
```

## Commands

See [COMMANDS.md](COMMANDS.md) for documentation on the commands available.

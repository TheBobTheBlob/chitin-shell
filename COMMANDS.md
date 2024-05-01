# Commands

- `file`: file manipulation
- `folder`: folder traversal
- `help`: in-shell help
- `math`: basic arthimethic and trignometry
- `string`: string manipulation
- `syscmd`: base OS shell link
- `if`: Control command chain continuation
- `exit`: exit the shell

Mandatory parameters are surrounded by square brackets `[]` and optional parameters are surrounded by curly brackets `{}`. These brackets should not be part of the command when using the shell.

## File

Create a file with the given file name

```console
file create [file_name]
```

Delete the file with the given file name

```console
file delete [file_name]
```

Read the contents of a file

```console
file read [file_name]
```

Completely replace the contents of a file

```console
file write [file_name] [text]
```

Append more text to the end of a file

```console
file append [file_name] [text]
```

Live edit a file in the shell

```console
file edit [file_name]
```

Running the command above will show some information about the file, a preview of lines around the selected line, and an input. The currently selected line is shown using a exclamation mark `!`, and the user input is denoted using an angle bracket `>`.

```console
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

## Folder

Move to the parent folder

```console
folder up
```

Move to a child folder. If no folder is given and there is only one child folder, that will automatically be selected.

```console
folder down {folder_name}
```

List the contents of the current folder

```console
folder list
```

Create a folder with the given name

```console
folder create [folder_name]
```

Delete a folder and all its contents

```console
folder delete [folder_name]
```

## Math

Add, subtract, multiply, or divide two numbers

```console
math add [num1] [num2]
math subtract [num1] [num2]
math multiply [num1] [num2]
math divide [num1] [num2]
```

Find the sine, cosine, or tangent of a value. The type can either be `degree` or `radian`

```console
math sin [number] [type]
math cos [number] [type]
math tan [number] [type]
```

Compare the value of two numbers. Returns "True" if the comparison is true, or "False" otherwise.

```console
math greater [num1] [num2]
math less [num1] [num2]
math equal [num1] [num2]
```

## String

Enter a string into the shell to use with piping

```console
string create [text]
```

Replace all occurances of a string in some text

```console
string replace [text] [string_to_replace] [replacement_text]
```

Returns "True" if text1 and text2 are the same, or "False" otherwise

```console
string compare [text1] [text2]
```

Returns "True" if text1 contains text2, or "False" otherwise

```console
string includes [text1] [text2]
```

## Syscmd

Send a command to the shell of the operating system. Some commands, like switching directories, may not work.

```console
syscmd [cmd] {args1} {arg2} ...
```

The arguments of the command should be given as separate parameters, and not as a single string. For example, `"ls -la"` will try to run a command named "la -la".

## If

```console
if true [boolean]
if false [boolean]
```

If given the positive boolean "True" will let the rest of the command chain continue. If given the negative boolean "False" will terminate the command chain.

```console
file read text.txt | string includes %str "Hello" | if true %str | string create "Contains Hello"
```

For example, the command chain above will print out `Contains Hello` if the file contains "Hello", or does not put output anything otherwise.

## Exit

Exit the shell

```console
exit
```

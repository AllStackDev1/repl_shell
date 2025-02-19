# Custom REPL in Rust 🦀

## Overview
Building a custom REPL (Read-Eval-Print Loop) in Rust is an excellent way to gain hands-on experience with system programming, asynchronous I/O, and command parsing. This project implements a simple shell that executes basic commands, providing an interactive environment for users to interact with their file system and execute simple operations.

## Features
Our Rust shell follows the standard REPL structure:

- **Read** – Captures user input from standard input (stdin).
- **Evaluate** – Parses input into a structured `Command` type and executes it.
- **Print** – Displays relevant output via standard output (stdout).
- **Loop** – Waits for the next command and repeats the cycle.

### Supported Commands
The shell currently supports the following commands:

| Command  | Description |
|----------|-------------|
| `echo`   | Print text to the console. |
| `ls`     | List files in the current directory. |
| `pwd`    | Display the present working directory. |
| `cd`     | Change directories. |
| `touch`  | Create a new file. |
| `rm`     | Remove a file. |
| `cat`    | Read and display file contents. |

## Installation
To run the REPL, ensure you have Rust installed. You can install Rust using [rustup](https://rustup.rs/):

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the repository and navigate into the project directory:

```sh
git clone https://github.com/your-username/rust-repl.git
cd rust-repl
```

Build and run the project:

```sh
cargo run
```

## Usage
Once inside the REPL, you can type commands just like in a standard shell:

```sh
> echo Hello, Rust!
Hello, Rust!
> pwd
/home/user/rust-repl
> ls
main.rs  Cargo.toml  README.md
> cd src
> pwd
/home/user/rust-repl/src
```

To exit the REPL, use `Ctrl + D` or type `exit`.

## Implementation Details
- The shell reads user input asynchronously to avoid blocking operations.
- Commands are tokenized and mapped to Rust functions for execution.
- Error handling ensures invalid commands provide meaningful feedback.

## Contributing
Contributions are welcome! If you'd like to improve the shell, feel free to fork the repository, create a feature branch, and submit a pull request.

## License
This project is licensed under the MIT License. See the `LICENSE` file for details.


# Rust Shell Implementation

A basic shell implementation in Rust that supports common shell features including built-in commands and external program execution.

## Features

- Built-in Commands:

  - `cd` - Change directory (supports absolute paths, relative paths, and `~` for home directory)
  - `pwd` - Print working directory
  - `echo` - Print arguments to stdout
  - `type` - Identify command type (builtin or external)
  - `exit` - Exit the shell (optionally with status code)

- External Program Execution
  - Executes programs in PATH
  - Supports command arguments
  - Handles program output correctly

## Prerequisites

Make sure you have Rust installed on your system. If not, you can install it from [rustup.rs](https://rustup.rs/).

## Running the Shell

1. Clone the repository:

```bash
git clone <repository-url>
cd <repository-name>
```

2. Make the run script executable:

```bash
chmod +x your_program.sh
```

3. Run the shell:

```bash
./your_program.sh
```

## Usage Examples

```bash
# Change directory
$ cd /usr/local
$ cd ~/Documents
$ cd ../

# Print working directory
$ pwd

# Echo text
$ echo Hello World!

# Check command type
$ type echo
echo is a shell builtin
$ type ls
ls is /bin/ls

# Exit the shell
$ exit
$ exit 1  # Exit with status code
```

## Implementation Details

The shell is implemented in Rust and features:

- Command parsing and execution
- Environment variable handling
- Path resolution
- Error handling
- Process management

Key Rust features used:

- Pattern matching
- Error handling with `Result` and `Option`
- File system operations
- Process spawning
- Environment variable access

## Development

The project uses standard Rust tooling. Build using:

```bash
cargo build
```


## Contributing


Feel free to submit issues and enhancement requests!

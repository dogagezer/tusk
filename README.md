# TUSK - Task Management CLI

TUSK is a simple command-line task management tool that allows you to organize your tasks efficiently across multiple accounts.

## Features

- **Add Task**: Add a new task to a specific account.
- **Delete Task**: Delete a task from an account.
- **Complete Task**: Mark a task as completed.
- **Uncomplete Task**: Mark a completed task as incomplete.
- **List Tasks**: List all tasks for a specific account.
- **Clear Tasks**: Clear all tasks for a specific account.

## Installation

To use TUSK, you need to have Rust and Cargo installed. Then, you can clone this repository and compile the code using Cargo.

```bash
git clone https://github.com/dogagezer/tusk.git
cd tusk
cargo build --release
```

## Usage

After compiling the code, you can run the TUSK CLI using the following command:

```bash
./target/debug/tusk [SUBCOMMAND]
```

### Subcommands
  ```
  tusk add --acc <account_name> --description "Task description"
  tusk delete --acc <account_name> --id <task_id>
  tusk complete --acc <account_name> --id <task_id>
  tusk uncomplete --acc <account_name> --id <task_id>
  tusk list --acc <account_name>
  tusk clear --acc <account_name>

  ```

### Examples

```bash
# Add a task to the "work" account
./target/debug/tusk add work "Prepare presentation slides"

# Mark task ID 3 in the "personal" account as completed
./target/debug/tusk complete personal 3

# List all tasks in the "work" account
./target/debug/tusk list work
```

## License

This project is licensed under the MIT License.

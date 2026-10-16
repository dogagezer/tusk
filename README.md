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

TUSK ships prebuilt binaries for Linux and macOS, so you do not need Rust or Cargo.

### Quick install (Linux / macOS)

```bash
curl -fsSL https://raw.githubusercontent.com/dogagezer/tusk/main/install.sh | bash
```

Or download the matching binary for your system from the
[Releases page](https://github.com/dogagezer/tusk/releases) and place it
on your `PATH`.

Available prebuilt binaries:

- `tusk-linux-x86_64`
- `tusk-linux-aarch64`
- `tusk-macos-x86_64`
- `tusk-macos-aarch64`

After downloading a binary directly, you can install it as the `tusk`
command with:

```bash
./tusk-macos-universal install
```

For macOS, a universal binary is always available directly from the
repository:

```bash
curl -fsSL https://raw.githubusercontent.com/dogagezer/tusk/main/releases/tusk-macos-universal -o tusk
chmod +x tusk
./tusk install
```

### Build from source (optional)

If you prefer to build locally you still can:

```bash
git clone https://github.com/dogagezer/tusk.git
cd tusk
cargo build --release
```

## Usage

With TUSK installed (or run as `./tusk` from the repository), use:

```bash
tusk [SUBCOMMAND]
```

### Subcommands
```
tusk add        <account_name> "<task description>"
tusk addp       <account_name> "<task description>" <high|medium|low>
tusk delete     <account_name> <task_id>
tusk complete   <account_name> <task_id>
tusk incomplete <account_name> <task_id>
tusk list       <account_name>
tusk clear      <account_name>
tusk install    Install tusk to your PATH
```

### Examples

```bash
# Add a task to the "work" account
tusk add work "Prepare presentation slides"

# Add a high-priority task
tusk addp work "Finish quarterly report" high

# Mark task ID 3 in the "personal" account as completed
tusk complete personal 3

# List all tasks in the "work" account
tusk list work
```

If TUSK is not yet on your `PATH`, run `./tusk install` or use the
one-line installer from the Installation section.

## Contributing

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for setup,
testing, and pull request guidelines. TUSK participates in Hacktoberfest;
issues labeled `hacktoberfest` and `good first issue` are a great place
to start.

## License

This project is licensed under the MIT License.

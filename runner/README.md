# Runner

A lightweight development tool for hot-reloading Rust executables during development.

## Overview

Runner is a file watcher that automatically restarts your executable whenever it detects changes to the binary. This enables a fast development workflow where you can build your application and have it automatically restart without manual intervention (primarily when developing with docker).

## How It Works

1. Takes a path to an executable as a command-line argument
2. Runs the executable in a child process
3. Watches the executable file for changes (create/modify events)
4. Automatically kills and restarts the process when changes are detected

## Usage

```bash
# Build the runner
cargo build

# Run an executable with auto-reload
./target/debug/runner ./target/debug/your-app

# Or use it with the app binary
./target/debug/runner ./target/debug/app
```

## Development Workflow

In the Docker development environment, the runner is used in conjunction with `cargo build` to provide hot-reloading:

1. Make changes to your source code
2. Run `make build` (or `cargo build` inside the container)
3. The runner detects the rebuilt binary and automatically restarts it
4. See your changes immediately without manually stopping/starting the server

## Example

```bash
# Terminal 1: Run your app with the runner
./target/debug/runner ./target/debug/app

# Terminal 2: Make changes and rebuild
cargo build
# The app automatically restarts in Terminal 1
```

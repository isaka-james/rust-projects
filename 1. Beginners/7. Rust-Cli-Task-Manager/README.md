# 📝 Usage Guide for Rust CLI Task Manager

### 📌 Add a Task
```sh
cargo run -- add "Complete Rust project"
```

### 📋 List All Tasks
```sh
cargo run -- list
```

### ✅ Mark a Task as Done
```sh
cargo run -- done 1
```

### ❌ Remove a Task
```sh
cargo run -- remove 1
```

### 📦 Running the Built Executable
After building the project:
```sh
target/release/cli-task-manager list
```

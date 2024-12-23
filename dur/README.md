# 📂 dur

`dur` is a terminal-based tool that visually analyzes directory sizes and provides an intuitive display of file and folder sizes.

## ✨ Features

- **🖥️ Intuitive Interface**: Displays directory structures and sizes using a TUI (Terminal User Interface).
- **⏱️ Real-Time Scanning**: Shows folder scanning status in real-time.
- **📂 Caching**: Caches size information to speed up subsequent scans.
- **🎯 Easy Navigation**: Navigate directories effortlessly with keyboard controls.
- **🚀 Handles Large Directories**: Efficiently processes large and complex directories.

## 🛠️ Installation

To use `dur`, you need the Rust toolchain. Follow these steps to install:

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/dur.git
   cd dur
   ```

2. Build and install:

   ```bash
   cargo install --path .
   ```

## 🏃 Usage

Run `dur` with the following command:

```bash
dur [PATH]
```

- `PATH` specifies the directory to analyze (defaults to the current directory `.`).

### 🔑 Keybindings

- `⬆️ / ⬇️`: Navigate between items
- `⬅️`: Go back to the parent directory
- `➡️ / Enter`: Open the selected directory
- `❌ q / ESC`: Quit the application

### 🖼️ Example Output

```text
📁 directory_name
  📄 file1.txt          ███░░░░░░░░░░░░░  1.5MB (15%)
  📄 file2.log          █████░░░░░░░░░░  5.0MB (50%)
  📁 subdir             ██████░░░░░░░░░  3.5MB (35%)
... and 3 other items
```

## 👩‍💻 Developer Information

### 🗂️ Source Code Structure

- `src/app.rs`: Application logic
- `src/scanner.rs`: Directory scanning and caching logic
- `src/ui/`: User interface-related code
- `src/types.rs`: Data structures and cache management

### ⚙️ Build and Test

1. Build:

   ```bash
   cargo build --release
   ```

2. Test:

   ```bash
   cargo test
   ```

## 🤝 Contribution

Bug reports and feature requests are welcome! Feel free to open an issue on GitHub to provide feedback.

## 📜 License

This project is licensed under the [MIT License](LICENSE).

# toi (問い) - Pipeline Confirmation Tool

`toi` (pronounced "toi", meaning "question" in Japanese) is a command-line tool designed to add an interactive confirmation step to Unix-style pipelines. It allows users to inspect the output of a command before passing it to the next command in the pipeline.

## Origin of the Name

The name `toi` comes from the Japanese word "問い" (toi), which means "question" or "inquiry". This reflects the tool's primary function of providing a confirmation prompt.

## Features

- Displays input content for review
- Interactive confirmation prompt
- Customizable timeout
- Default yes/no options
- Custom prompt messages
- Works seamlessly in complex pipelines

## Installation

### Prerequisites

- Go 1.16 or later

### Building from source

1. Clone the repository:
   ```
   git clone https://github.com/nwiizo/toi.git
   cd toi
   ```

2. Build the binary:
   ```
   go build -o toi
   ```

3. (Optional) Move the binary to a directory in your PATH:
   ```
   sudo mv toi /usr/local/bin/
   ```

## Usage

Basic syntax:

```
command1 | toi [flags] | command2
```

### Flags

- `-t, --timeout int`: Set a timeout in seconds (0 for no timeout)
- `-y, --yes`: Default to yes if no input is provided
- `-n, --no`: Default to no if no input is provided
- `-p, --prompt string`: Set a custom prompt message

## Examples

1. Basic usage:
   ```
   ls | toi | wc -l
   ```
   This will display the output of `ls`, prompt for confirmation, and if approved, count the number of lines.

2. Using default yes:
   ```
   echo "Hello, World!" | toi -y | tr '[:lower:]' '[:upper:]'
   ```
   This will convert the input to uppercase without prompting, due to the `-y` flag.

3. With timeout:
   ```
   cat /etc/passwd | toi -t 5 | grep root
   ```
   This sets a 5-second timeout for the confirmation prompt.

4. Custom prompt:
   ```
   ps aux | toi -p "Do you want to see the process list? (y/n): " | awk '{print $2, $11}'
   ```
   This uses a custom prompt message before displaying the process list.

5. Complex pipeline:
   ```
   find . -type f | toi | xargs -I {} sh -c 'echo "Processing: {}"; wc -l {}'
   ```
   This finds all files in the current directory, confirms, then counts lines in each file.


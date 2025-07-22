# Jenda
A command line tool for task tracking.

#### Philosophy
I spend most of my day in the terminal.
While I am working, I often have stray thoughts that need to be written down.
I like the simplicity (and therefore speed) of writing them on paper.
Unfortunately, most of these papers get misplaced and thrown away.
Jenda aims to replace random papers while maintaining simplicity.

## Features
- Create tasks with
  - Name
  - Complete flag
  - Timestamp
- Recall tasks by
  - Substring of name
  - Complete flag
- Mark tasks as complete

## Usage
```
Usage: jenda [COMMAND]

Commands:
  add   Add a new task
  ct    Mark all tasks in group as complete
  list  List all tasks in a task group
  info  Display info for a single task
  help  Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

### Tags
While Jenda does not have explict infrastructure for tags, the behaviour can be
derived by enforcing a naming policy.

```
jenda add -n "work: update documentation"
jenda list -n "work:"
```


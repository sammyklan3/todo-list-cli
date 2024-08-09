# To-Do List CLI Application

This is a simple command-line interface (CLI) application written in Rust that allows you to manage a to-do list. Tasks can be added, removed, marked as completed, and listed. The tasks are saved to a JSON file for persistence.

## Features

- **Add Tasks:** Easily add tasks with a description to your to-do list.
- **Remove Tasks:** Remove tasks from the list using their unique ID.
- **Mark Tasks as Completed:** Mark tasks as completed by their unique ID.
- **List Tasks:** View all tasks with their completion status.
- **Persistent Storage:** Tasks are saved to a JSON file (`todo_list.json`) so they persist across sessions.

## Prerequisites

To run this project, you need to have Rust installed. You can install Rust by following the instructions at [rust-lang.org](https://www.rust-lang.org/learn/get-started).

## Getting Started

### 1. Clone the repository
```bash
git clone https://github.com/sammyklan3/todo-list-cli.git
cd todo-list-cli
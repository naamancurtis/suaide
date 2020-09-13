# Suaide _(Stand-Up Aide)_

A simple CLI App written purely in Rust to help with the management of todos & tasks within the terminal,
with the added functionality to output a stand-up style report.

## Requirements

Under the hood this uses `SQLite 3` for storage, _(included in most MacOS
distributions)_. If your system doesn't have it installed you'll need to install
a recent version of it _(this was developed on 3.28.0)_.

## Basic API

| Command   | Description                    |
| --------- | ------------------------------ |
| `add`     | Adds a new task                |
| `edit`    | Edit a task                    |
| `list`    | Lists all tasks                |
| `remove`  | Deletes a task                 |
| `close`   | Marks a task as closed         |
| `status`  | Change the status of a task    |
| `standup` | Prints out the stand-up output |

# suaide

[![Build Status](https://api.travis-ci.com/naamancurtis/suaide.svg?branch=master)](https://travis-ci.com/naamancurtis/suaide) [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

_[Stand-up Aide]_ A lightweight 100% offline CLI App to help with the management of todos & tasks within the terminal,
with the added functionality to output a stand-up style report.

It's very much in an early _alpha_ testing stage, so please raise an issue if you
spot anything or have a feature request.

## Requirements

Under the hood this uses `SQLite 3` for storage, _(included in most MacOS
distributions)_. If your system doesn't have it installed you'll need to install
a recent version of it _(this was developed on `SQLite v3.28.0`)_.

## Basic API

| Command   | Description                    | Example             |
| --------- | ------------------------------ | ------------------- |
| `add`     | Adds a new task                | `suaide add`        |
| `edit`    | Edit a task                    | `suaide edit 123`   |
| `list`    | Lists all tasks                | `suaide list -a`    |
| `remove`  | Deletes a task                 | `suaide remove 123` |
| `close`   | Marks a task as closed         | `suaide remove 123` |
| `status`  | Change the status of a task    | `suaide status 123` |
| `standup` | Prints out the stand-up output | `suaide standup`    |

## Settings

There are two options for overwriting default settings for **suaide**: _(in order of
precedence)_

1. Environment variables prefixed with `SUAIDE_`
2. A config file located at `~/.suaide/settings.yml`

### Options

| Setting         | Description                                                             | Default     |
| --------------- | ----------------------------------------------------------------------- | ----------- |
| `db_url`        | The path to the `suaide.sqlite` file used as the database for the tasks | `~/.suaide` |
| `ticket_prefix` | A prefix that will be automatically applied to any ticket id if set     | `""`        |

#### Example settings.yml

```yml
db_url: ~/code/todos
ticket_prefix: TASK-
```

#### Example Environment Variables

```
SUAIDE_DB_URL="~/code/todos"
```

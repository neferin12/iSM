oclif-hello-world
=================

oclif example Hello World CLI

[![oclif](https://img.shields.io/badge/cli-oclif-brightgreen.svg)](https://oclif.io)
[![CircleCI](https://circleci.com/gh/oclif/hello-world/tree/main.svg?style=shield)](https://circleci.com/gh/oclif/hello-world/tree/main)
[![GitHub license](https://img.shields.io/github/license/oclif/hello-world)](https://github.com/oclif/hello-world/blob/main/LICENSE)

<!-- toc -->
* [Usage](#usage)
* [Commands](#commands)
<!-- tocstop -->
# Usage
<!-- usage -->
```sh-session
$ npm install -g tsism
$ tsism COMMAND
running command...
$ tsism (--version)
tsism/1.0.1 linux-x64 node-v18.16.0
$ tsism --help [COMMAND]
USAGE
  $ tsism COMMAND
...
```
<!-- usagestop -->
# Commands
<!-- commands -->
* [`tsism autocomplete [SHELL]`](#tsism-autocomplete-shell)
* [`tsism help [COMMANDS]`](#tsism-help-commands)
* [`tsism run STUDENTS SEMINARS RUNS`](#tsism-run-students-seminars-runs)

## `tsism autocomplete [SHELL]`

display autocomplete installation instructions

```
USAGE
  $ tsism autocomplete [SHELL] [-r]

ARGUMENTS
  SHELL  (zsh|bash|powershell) Shell type

FLAGS
  -r, --refresh-cache  Refresh cache (ignores displaying instructions)

DESCRIPTION
  display autocomplete installation instructions

EXAMPLES
  $ tsism autocomplete

  $ tsism autocomplete bash

  $ tsism autocomplete zsh

  $ tsism autocomplete powershell

  $ tsism autocomplete --refresh-cache
```

_See code: [@oclif/plugin-autocomplete](https://github.com/oclif/plugin-autocomplete/blob/v2.2.0/src/commands/autocomplete/index.ts)_

## `tsism help [COMMANDS]`

Display help for tsism.

```
USAGE
  $ tsism help [COMMANDS] [-n]

ARGUMENTS
  COMMANDS  Command to show help for.

FLAGS
  -n, --nested-commands  Include all nested commands in the output.

DESCRIPTION
  Display help for tsism.
```

_See code: [@oclif/plugin-help](https://github.com/oclif/plugin-help/blob/v5.2.9/src/commands/help.ts)_

## `tsism run STUDENTS SEMINARS RUNS`

Run the algorithm

```
USAGE
  $ tsism run STUDENTS SEMINARS RUNS

ARGUMENTS
  STUDENTS  The file containing the students
  SEMINARS  The file containing the seminars
  RUNS      Number of iterations of the algorithm

DESCRIPTION
  Run the algorithm
```

_See code: [dist/commands/run/index.ts](https://github.com/neferin12/iSM/blob/v1.0.1/dist/commands/run/index.ts)_
<!-- commandsstop -->

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
tsism/0.0.0 linux-x64 node-v18.16.0
$ tsism --help [COMMAND]
USAGE
  $ tsism COMMAND
...
```
<!-- usagestop -->
# Commands
<!-- commands -->
* [`tsism autocomplete [SHELL]`](#tsism-autocomplete-shell)
* [`tsism hello PERSON`](#tsism-hello-person)
* [`tsism hello world`](#tsism-hello-world)
* [`tsism help [COMMANDS]`](#tsism-help-commands)
* [`tsism plugins`](#tsism-plugins)
* [`tsism plugins:install PLUGIN...`](#tsism-pluginsinstall-plugin)
* [`tsism plugins:inspect PLUGIN...`](#tsism-pluginsinspect-plugin)
* [`tsism plugins:install PLUGIN...`](#tsism-pluginsinstall-plugin-1)
* [`tsism plugins:link PLUGIN`](#tsism-pluginslink-plugin)
* [`tsism plugins:uninstall PLUGIN...`](#tsism-pluginsuninstall-plugin)
* [`tsism plugins:uninstall PLUGIN...`](#tsism-pluginsuninstall-plugin-1)
* [`tsism plugins:uninstall PLUGIN...`](#tsism-pluginsuninstall-plugin-2)
* [`tsism plugins update`](#tsism-plugins-update)

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

## `tsism hello PERSON`

Say hello

```
USAGE
  $ tsism hello PERSON -f <value>

ARGUMENTS
  PERSON  Person to say hello to

FLAGS
  -f, --from=<value>  (required) Who is saying hello

DESCRIPTION
  Say hello

EXAMPLES
  $ oex hello friend --from oclif
  hello friend from oclif! (./src/commands/hello/index.ts)
```

_See code: [dist/commands/hello/index.ts](https://github.com/neferin12/iSM/blob/v0.0.0/dist/commands/hello/index.ts)_

## `tsism hello world`

Say hello world

```
USAGE
  $ tsism hello world

DESCRIPTION
  Say hello world

EXAMPLES
  $ tsism hello world
  hello world! (./src/commands/hello/world.ts)
```

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

## `tsism plugins`

List installed plugins.

```
USAGE
  $ tsism plugins [--core]

FLAGS
  --core  Show core plugins.

DESCRIPTION
  List installed plugins.

EXAMPLES
  $ tsism plugins
```

_See code: [@oclif/plugin-plugins](https://github.com/oclif/plugin-plugins/blob/v2.4.7/src/commands/plugins/index.ts)_

## `tsism plugins:install PLUGIN...`

Installs a plugin into the CLI.

```
USAGE
  $ tsism plugins:install PLUGIN...

ARGUMENTS
  PLUGIN  Plugin to install.

FLAGS
  -f, --force    Run yarn install with force flag.
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Installs a plugin into the CLI.
  Can be installed from npm or a git url.

  Installation of a user-installed plugin will override a core plugin.

  e.g. If you have a core plugin that has a 'hello' command, installing a user-installed plugin with a 'hello' command
  will override the core plugin implementation. This is useful if a user needs to update core plugin functionality in
  the CLI without the need to patch and update the whole CLI.


ALIASES
  $ tsism plugins add

EXAMPLES
  $ tsism plugins:install myplugin 

  $ tsism plugins:install https://github.com/someuser/someplugin

  $ tsism plugins:install someuser/someplugin
```

## `tsism plugins:inspect PLUGIN...`

Displays installation properties of a plugin.

```
USAGE
  $ tsism plugins:inspect PLUGIN...

ARGUMENTS
  PLUGIN  [default: .] Plugin to inspect.

FLAGS
  -h, --help     Show CLI help.
  -v, --verbose

GLOBAL FLAGS
  --json  Format output as json.

DESCRIPTION
  Displays installation properties of a plugin.

EXAMPLES
  $ tsism plugins:inspect myplugin
```

## `tsism plugins:install PLUGIN...`

Installs a plugin into the CLI.

```
USAGE
  $ tsism plugins:install PLUGIN...

ARGUMENTS
  PLUGIN  Plugin to install.

FLAGS
  -f, --force    Run yarn install with force flag.
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Installs a plugin into the CLI.
  Can be installed from npm or a git url.

  Installation of a user-installed plugin will override a core plugin.

  e.g. If you have a core plugin that has a 'hello' command, installing a user-installed plugin with a 'hello' command
  will override the core plugin implementation. This is useful if a user needs to update core plugin functionality in
  the CLI without the need to patch and update the whole CLI.


ALIASES
  $ tsism plugins add

EXAMPLES
  $ tsism plugins:install myplugin 

  $ tsism plugins:install https://github.com/someuser/someplugin

  $ tsism plugins:install someuser/someplugin
```

## `tsism plugins:link PLUGIN`

Links a plugin into the CLI for development.

```
USAGE
  $ tsism plugins:link PLUGIN

ARGUMENTS
  PATH  [default: .] path to plugin

FLAGS
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Links a plugin into the CLI for development.
  Installation of a linked plugin will override a user-installed or core plugin.

  e.g. If you have a user-installed or core plugin that has a 'hello' command, installing a linked plugin with a 'hello'
  command will override the user-installed or core plugin implementation. This is useful for development work.


EXAMPLES
  $ tsism plugins:link myplugin
```

## `tsism plugins:uninstall PLUGIN...`

Removes a plugin from the CLI.

```
USAGE
  $ tsism plugins:uninstall PLUGIN...

ARGUMENTS
  PLUGIN  plugin to uninstall

FLAGS
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Removes a plugin from the CLI.

ALIASES
  $ tsism plugins unlink
  $ tsism plugins remove
```

## `tsism plugins:uninstall PLUGIN...`

Removes a plugin from the CLI.

```
USAGE
  $ tsism plugins:uninstall PLUGIN...

ARGUMENTS
  PLUGIN  plugin to uninstall

FLAGS
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Removes a plugin from the CLI.

ALIASES
  $ tsism plugins unlink
  $ tsism plugins remove
```

## `tsism plugins:uninstall PLUGIN...`

Removes a plugin from the CLI.

```
USAGE
  $ tsism plugins:uninstall PLUGIN...

ARGUMENTS
  PLUGIN  plugin to uninstall

FLAGS
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Removes a plugin from the CLI.

ALIASES
  $ tsism plugins unlink
  $ tsism plugins remove
```

## `tsism plugins update`

Update installed plugins.

```
USAGE
  $ tsism plugins update [-h] [-v]

FLAGS
  -h, --help     Show CLI help.
  -v, --verbose

DESCRIPTION
  Update installed plugins.
```
<!-- commandsstop -->

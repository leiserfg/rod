# rod: Dark/Light Mode Detection Tool

Tool for [Dark and Light Mode detection](https://github.com/contour-terminal/contour/blob/master/docs/vt-extensions/color-palette-update-notifications.md#dark-and-light-mode-detection)

## How to Use rod

`rod` offers three ways to integrate with your terminal applications: aliases, direct printing, and environment variable manipulation.

### 1. Using Aliases

This method works across major shells and is suitable for commands that need extra arguments to set themes.

```sh
alias fzf=rod run fzf
```


### 2. Using `rod print` for scripting

For scripting needs, use the `rod print` command to retrieve the current color scheme.


### 3. Using `rod env` for Environment Variable-Based Applications

This method is ideal for applications that read theme settings from environment variables, it needs to be configured shell-specifically, as follows:

#### fish:

```fish
function preexec_rod --on-event fish_preexec
    env rod | source
endy
```

#### zsh:
```bash
preexec () {
    env rod | source
}
```

#### bash

Sadly there is not a proper `preexec` mechanism in bash. So to use this method within it, you need to either:
- Install [bash-preexec](https://github.com/rcaloras/bash-preexec) which will be something like:

```bash
preexec () {
    env rod | source
}
```

- Use a bash trap:
```bash
rod_trap () {
    [ -n "$COMP_LINE" ] && return  # do nothing if completing
    [ "$BASH_COMMAND" = "$PROMPT_COMMAND" ] && return # don't run for $PROMPT_COMMAND
    rod env|source
}
trap 'rod_trap' DEBUG
```

## Installation
You can find pre-build binaries in the releases.


gin
------------
gin (short for `git init`) is a convenience wrapper for `git config` and `git init`. It is useful for users with multiple Git configurations. It enables initializing a Git repository with a specific configuration, and switching configurations on the fly.

### Usage
Currently the only Git config location that is support is `XDG_CONFIG_HOME/git/`. The tool relies on `<name>.conf` files in this directory for changing the local configuration.
```
$ gin -p <name>
```

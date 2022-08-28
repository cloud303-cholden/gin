gin
------------
gin (short for `git init`) is a convenience wrapper for `git config` and (soon) `git init`. It is useful for users with multiple Git configurations. It enables (soon) initializing a Git repository with a specific configuration as well as switching configurations on the fly.

### Usage
Currently the only Git configuration location that is supported is `$XDG_CONFIG_HOME/git/`. The tool relies on `<name>.conf` files in this directory for changing the local configuration. The below command updates the `./.git/config` file and depends on the user being in the project root.
```
$ gin -p <name>
```

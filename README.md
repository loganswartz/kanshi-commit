# kanshi-commit

Commit the current display configuration to a kanshi config file.

## About

Oftentimes, it's not very convenient to fine-tune your sway/kanshi display
configuration directly; It's much easier to tweak your configuration via a
graphical display configuration tool like `wdisplays` instead. `kanshi-commit`
is intended to make it easy to commit the current display configuration directly
to a kanshi config file, to save you the hassle of manually transcribing the
configuration.

## Install

You'll need a working and up-to-date Cargo installation on your machine. Then,
you can install directly from this git repo:

```bash
# installs to ~/.cargo/bin/kanshi-commit by default
cargo install --git https://github.com/loganswartz/kanshi-commit
```

Alternatively, you can clone the repo locally, and then build via `cargo build
--release`.

## Usage

Run `kanshi-commit -h` for a full rundown of available options.

`kanshi-commit` is built around the expectation that your main kanshi config
looks something like:

```
include ~/.config/kanshi/config.d/*
```

This way, every machine you use can have any number of allowed configurations,
and each configuration can be described by their own standalone config file.

`kanshi-commit` requires a single argument, which is a profile name to use for
the generated configuration:

```bash
kanshi-commit my-new-config
```

`kanshi-commit` can optionally take a `--save` option, and if specified, the
generated config is automatically saved to a file. By default, the filename will
look like `<profile-name><suffix>`, where `<profile-name>` is whatever name was
passed to `kanshi-commit`, and `<suffix>` defaults to `.conf` (configurable via
the `--suffix` flag). This file is then placed in the appropriate config
directory, which is usually `$XDG_CONFIG_HOME/kanshi/config.d/` (this is
configurable via `--config-dir`). You can also inspect the path that a profile
would be saved to using the `--location` flag.

If a config already exists at the save location when using `--save`,
`kanshi-commit` will default to throwing an error and aborting, rather than
potentially overwriting the existing config. If `--force` is specified, this
safety check will be ignored, and it will overwrite any preexisting config with
the same name.

If `--save` is not specified, the profile is echoed to stdout.

## Examples

```bash
# dump the current display configuration as a kanshi profile named "my-config"
# to stdout
kanshi-commit my-config
```

```bash
# read the current configuration from a file
kanshi-commit my-config --from-file ~/configuration.json
# reading from stdin is also possible
swaymsg -t get_outputs --raw | kanshi-commit my-config --from-file -
# if --from-file is not specified, kanshi-commit attempts to run `swaymsg -t
# get_outputs --raw` and will use that configuration
```

```bash
# save to `$XDG_CONFIG_HOME/kanshi/config.d/my-config.conf`, overwriting if it
# already exists
kanshi-commit my-config --save --force
```

```bash
# save to `$XDG_CONFIG_HOME/kanshi/config.d/my-config` (no ".conf" suffix)
kanshi-commit my-config --save --force --suffix ""
```

```bash
# save to `/etc/kanshi/my-config.kanshi`, with no overwriting
kanshi-commit my-config --save --suffix ".kanshi" --config-dir "/etc/kanshi/"
```

```bash
# show where the config would be saved if --save were used
kanshi-commit my-config --location

# this flag also works with any other flags that affect the save location
kanshi-commit my-config --location --suffix ".kanshi" --config-dir "/etc/kanshi/"

# this does NOT save the config, as --location takes precedence and turns --save
# into a no-op
kanshi-commit my-config --location --save
```

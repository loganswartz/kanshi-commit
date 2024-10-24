# kanshi-commit

Commit the current display configuration to a kanshi config file.

## About

Oftentimes, it's not very convenient to fine-tune your sway/kanshi display
configuration directly; It's much easier to tweak your configuration via a
graphical display configuration tool like `wdisplays` instead. `kanshi-commit`
is intended to make it easy to commit the current display configuration directly
to a kanshi config file, to save you the hassle of manually transcribing the
configuration.

## Build

Build like any typical Rust program, with `cargo build`.

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
generated config is saved to `~/.config/kanshi/config.d/<profile-name>.conf`.
Otherwise, the profile is echoed to stdout. If `--force` is specified, it will
also overwrite a config if it already exists.

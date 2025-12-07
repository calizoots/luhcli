# luhcli
[<img alt="github" src="https://img.shields.io/badge/github-calizoots/luhcli-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/calizoots/luhcli)
[<img alt="crates.io" src="https://img.shields.io/crates/v/luhcli.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/luhcli)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-luhcli-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/luhcli)

A simple and ergonomic CLI library for Rust.
<br>
> made with love by s.c

## Core Ideas

`luhcli` aims to make building command-line interfaces straightforward, with full support for:
- Flags (`-h` / `--help`)
- Options (flags that take values)
- Positional arguments
- Variadic arguments (capture remaining arguments)
- Conditional sub-arguments

## Getting Started

Add `luhcli` to your `Cargo.toml`

```toml
[dependencies]
luhcli = "0.1"
```

Then, create a CLI app using `CliApp`, define commands and arguments, and run it:

```rust
use luhcli::{CliApp, Command, Arg};
use luhtwin::LuhTwin;

fn main() -> LuhTwin<()> {
    let cli = CliApp::new("myapp")
        .about("Example CLI application")
        .subcommand(
            Command::new("config")
                .about("Manage app configuration")
                .arg(
                    Arg::positional("action", 0)
                        .help("Action to perform")
                        .possible_values(["get", "set"])
                )
        );

    cli.run()?;
    Ok(())
}
```

## Examples

### Simple Flag

```rust
let verbose = Arg::new("verbose")
    .short('v')
    .long("verbose")
    .help("Enable verbose output");
```

### Positional Argument

```rust
let filename = Arg::positional("filename", 0)
    .help("Input file path");
```

### Option with Default Value

```rust
let output = Arg::new("output")
    .takes_value()
    .long("output")
    .default_value("out.txt")
    .help("Output file");
```

### Conditional Sub-Arguments

```rust
let mode_arg = Arg::new("mode")
    .takes_value()
    .when("advanced", vec![
        Arg::new("config").takes_value().help("Advanced config file")
    ]);
```

### All together

```rust
fn handle_config_command(args: &ParsedArgs) -> LuhTwin<()> {
    match args.get("action").map(|s| s.as_str()) {
        Some("list") => {
            // do something
        }

        // more here

        _ => {
            exit(3);
        }
    }
    Ok(())
}

fn handle_app_command(args: &ParsedArgs) -> LuhTwin<()> {
    match args.get("action").map(|s| s.as_str()) {
        Some("start") => {
            // do something
        }

        // more here

        _ => {
            exit(3);
        }
    }
    Ok(())
}

fn main() -> LuhTwin<() {
    let cli = App::new("moth")
        .about("your favourite music player <3")
        .subcommand(
            Command::new("config")
                .about("edit the moth.json config (has all the general options)")
                .arg(
                    Arg::positional("action", 0)
                        .help("action to perform")
                        .possible_values(["get", "set", "list"])
                        .when("get", vec![
                            Arg::positional("key", 0).help("key to retrieve")
                        ])
                        .when("set", vec![
                            Arg::positional("key", 0).help("key to set"),
                            Arg::positional("value", 1).help("value to set").required(false),
                        ])
                )
                .handler(handle_config_command)
        )
        .subcommand(
            Command::new("app")
                .about("start, stop, restart or list info about the app >.<")
                .arg(
                    Arg::positional("action", 0)
                        .help("action to perform")
                        .possible_values(["start", "stop", "restart", "info"])
                        .when("info", vec![
                            Arg::new("noformat")
                                .short('n')
                                .long("noformat")
                                .help("No log formatting in info")
                        ])
                )
                .handler(handle_app_command)
        );

    cli.run()?;
}
```

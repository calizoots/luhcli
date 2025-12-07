//! [![github]](https://github.com/calizoots/luhcli)&ensp;[![crates-io]](https://crates.io/crates/luhcli)&ensp;[![docs-rs]](https://docs.rs/luhcli)
//!
//! [github]: https://img.shields.io/badge/github-calizoots/luhcli-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/crates/v/luhcli.svg?style=for-the-badge&color=fc8d62&logo=rust
//! [docs-rs]: https://img.shields.io/badge/docs.rs-luhcli-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs
//!
//! # luhcli
//!
//! A simple and ergonomic CLI library for Rust.
//! > made with love by s.c
//!
//! ## Core Ideas
//!
//! `luhcli` aims to make building command-line interfaces straightforward, with full support for:
//! - Flags (`-h` / `--help`)
//! - Options (flags that take values)
//! - Positional arguments
//! - Variadic arguments (capture remaining arguments)
//! - Conditional sub-arguments
//!
//! ## Getting Started
//!
//! Add `luhcli` to your `Cargo.toml`
//!
//! ```toml
//! [dependencies]
//! luhcli = "0.1"
//! ```
//!
//! Then, create a CLI app using `CliApp`, define commands and arguments, and run it:
//!
//! ```ignore
//! use luhcli::{CliApp, Command, Arg};
//! use luhtwin::LuhTwin;
//!
//! fn main() -> LuhTwin<()> {
//!     let cli = CliApp::new("myapp")
//!         .about("Example CLI application")
//!         .subcommand(
//!             Command::new("config")
//!                 .about("Manage app configuration")
//!                 .arg(
//!                     Arg::positional("action", 0)
//!                         .help("Action to perform")
//!                         .possible_values(["get", "set"])
//!                 )
//!         );
//!
//!     cli.run()?;
//!     Ok(())
//! }
//! ```
//!
//! ## Examples
//!
//! ### Simple Flag
//!
//! ```ignore
//! let verbose = Arg::new("verbose")
//!     .short('v')
//!     .long("verbose")
//!     .help("Enable verbose output");
//! ```
//!
//! ### Positional Argument
//!
//! ```ignore
//! let filename = Arg::positional("filename", 0)
//!     .help("Input file path");
//! ```
//!
//! ### Option with Default Value
//!
//! ```ignore
//! let output = Arg::new("output")
//!     .takes_value()
//!     .long("output")
//!     .default_value("out.txt")
//!     .help("Output file");
//! ```
//!
//! ### Conditional Sub-Arguments
//!
//! ```ignore
//! let mode_arg = Arg::new("mode")
//!     .takes_value()
//!     .when("advanced", vec![
//!         Arg::new("config").takes_value().help("Advanced config file")
//!     ]);
//! ```
//!

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use luhlog::error;
use luhtwin::{LuhTwin, at};

/// Can either be Flag, Option, Positional or Variadic
///   - Flag (being -h, --help)
///   - Option being a flag which takes a value
///   - Positional being dependant of index of the arguments
///   - Variadic capturing the remaning args
/// 
/// ## note
///
/// You typically wouldn't use this by itself when construcing args
/// you would use predefined methods please see `luhcli::Arg`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArgType {
    /// Flag (e.g -h, --help)
    Flag,
    /// Option is a flag which takes a value
    Option,
    /// Positional is dependant of index of the arguments
    Positional { index: usize },
    /// Variadic captures the remaning args
    Variadic,
}

/// Represents a single command-line argument.
///
/// `Arg` is the core building block for defining CLI arguments in `luhcli`.
/// It supports flags, options (arguments that take a value), positional arguments, and variadic arguments.
///
/// You can chain methods to configure its properties, such as `short`, `long`, `help`, `required`, and `default_value`.
///
/// # Examples
///
/// ```ignore
/// use luhcli::Arg;
///
/// // A simple flag
/// let verbose = Arg::new("verbose").short('v').help("Enable verbose output");
///
/// // A positional argument
/// let filename = Arg::positional("filename", 0).help("Input file");
///
/// // An option with a default value
/// let output = Arg::new("output")
///     .takes_value()
///     .long("output")
///     .default_value("out.txt");
/// ```
/// # Provided Methods
///
/// - [`Arg::new`] – Create a new flag argument.
/// - [`Arg::positional`] – Create a new positional argument.
/// - [`Arg::variadic`] – Create a new variadic argument.
/// - [`Arg::short`] – Set a short flag (e.g., `-h`).
/// - [`Arg::long`] – Set a long flag (e.g., `--help`).
/// - [`Arg::help`] – Set the help message.
/// - [`Arg::required`] – Mark the argument as required.
/// - [`Arg::takes_value`] – Mark the argument as an option that takes a value.
/// - [`Arg::depends_on`] – Add dependencies on other arguments.
/// - [`Arg::conflicts_with`] – Add conflicts with other arguments.
/// - [`Arg::default_value`] – Set a default value.
/// - [`Arg::possible_values`] – Restrict allowed values.
/// - [`Arg::when`] – Define conditional sub-arguments.
#[derive(Clone)]
pub struct Arg {
    /// Name of the argument (used internally and as default for long option)
    pub name: String,
    /// Type of the argument (Flag, Option, Positional, or Variadic)
    pub arg_type: ArgType,
    /// Optional single-character short flag (e.g., `-h`)
    pub short: Option<char>,
    /// Optional long flag (e.g., `--help`)
    pub long: Option<String>,
    /// Help message describing the argument
    pub help: String,
    /// Whether the argument is required
    pub required: bool,
    /// List of other arguments that this argument depends on
    pub depends_on: Vec<String>,
    /// List of arguments that conflict with this argument
    pub conflicts_with: Vec<String>,
    /// Optional default value for the argument
    pub default_value: Option<String>,
    /// List of allowed values for this argument
    pub possible_values: Vec<String>,
    /// Conditional sub-arguments that apply when this argument has a specific value
    pub children: Vec<ArgChain>,
}

/// Represents a set of sub-arguments that are only active when the parent `Arg` has a specific value.
///
/// Useful for defining complex argument relationships, such as conditional options.
///
/// # Example
///
/// ```ignore
/// use luhcli::Arg;
///
/// let mode_arg = Arg::new("mode")
///     .takes_value()
///     .when("advanced", vec![
///         Arg::new("config").takes_value().help("Advanced config file")
///     ]);
/// ```
#[derive(Clone)]
pub struct ArgChain {
    /// The value of the parent argument that triggers this chain
    pub when_value: String,
    /// Arguments that become active when the parent argument matches `when_value`
    pub args: Vec<Arg>,
}

impl Arg {
    /// Create a new flag argument.
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            name: name.clone(),
            short: None,
            long: Some(name.clone()),
            help: String::new(),
            required: false,
            arg_type: ArgType::Flag,
            depends_on: Vec::new(),
            conflicts_with: Vec::new(),
            default_value: None,
            possible_values: Vec::new(),
            children: Vec::new(),
        }
    }
    
    /// Create a new positional argument.
    pub fn positional(name: impl Into<String>, index: usize) -> Self {
        Self {
            name: name.into(),
            short: None,
            long: None,
            help: String::new(),
            required: true,
            arg_type: ArgType::Positional { index },
            depends_on: Vec::new(),
            conflicts_with: Vec::new(),
            default_value: None,
            possible_values: Vec::new(),
            children: Vec::new(),
        }
    }
    
    /// Create a new variadic argument.
    pub fn variadic(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            short: None,
            long: None,
            help: String::new(),
            required: false,
            arg_type: ArgType::Variadic,
            depends_on: Vec::new(),
            conflicts_with: Vec::new(),
            default_value: None,
            possible_values: Vec::new(),
            children: Vec::new(),
        }
    }
    
    /// Set a short flag (e.g., `-h`).
    pub fn short(mut self, c: char) -> Self {
        self.short = Some(c);
        self
    }
    
    /// Set a long flag (e.g., `--help`).
    pub fn long(mut self, s: impl Into<String>) -> Self {
        self.long = Some(s.into());
        self
    }
    
    /// Set the help message.
    pub fn help(mut self, h: impl Into<String>) -> Self {
        self.help = h.into();
        self
    }
    
    /// Mark the argument as required.
    pub fn required(mut self, r: bool) -> Self {
        self.required = r;
        self
    }
    
    /// Mark the argument as an option that takes a value.
    pub fn takes_value(mut self) -> Self {
        self.arg_type = ArgType::Option;
        self
    }
    
    /// Add dependencies on other arguments.
    pub fn depends_on(mut self, arg: impl Into<String>) -> Self {
        self.depends_on.push(arg.into());
        self
    }
    
    /// Add conflicts with other arguments.
    pub fn conflicts_with(mut self, arg: impl Into<String>) -> Self {
        self.conflicts_with.push(arg.into());
        self
    }
    
    /// Set a default value.
    pub fn default_value(mut self, val: impl Into<String>) -> Self {
        self.default_value = Some(val.into());
        self
    }
    
    /// Restrict allowed values.
    pub fn possible_values<I, S>(mut self, values: I) -> Self 
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.possible_values = values.into_iter().map(Into::into).collect();
        self
    }
    
    /// Define conditional sub-arguments.
    pub fn when<I>(mut self, value: impl Into<String>, args: I) -> Self 
    where
        I: IntoIterator<Item = Arg>,
    {
        self.children.push(ArgChain {
            when_value: value.into(),
            args: args.into_iter().collect(),
        });
        self
    }
}

/// Represents the result of parsing command-line arguments with `luhcli`.
///  
/// Stores values for options, flags, positional arguments, and variadic arguments.
///
/// Provides convenient accessors for retrieving arguments after parsing.
///
/// # Examples
///
/// ```ignore
/// use luhcli::ParsedArgs;
///
/// // Imagine `parsed` is the result of parsing CLI arguments
/// let parsed: ParsedArgs = /* parsing logic */;
///
/// // Retrieve a value of an option
/// if let Some(output) = parsed.get("output") {
///     println!("Output file: {}", output);
/// }
///
/// // Check if a flag was set
/// if parsed.flag("verbose") {
///     println!("Verbose mode enabled");
/// }
///
/// // Access positional arguments
/// for arg in parsed.positional() {
///     println!("Positional argument: {}", arg);
/// }
///
/// // Access variadic arguments
/// for arg in parsed.variadic() {
///     println!("Variadic argument: {}", arg);
/// }
/// ```
/// 
/// # Provided Methods
///
/// - [`ParsedArgs::get`] – Retrieve the value of an option by name.
/// - [`ParsedArgs::flag`] – Check if a flag was set.
/// - [`ParsedArgs::positional`] – Get a slice of all positional arguments.
/// - [`ParsedArgs::variadic`] – Get a slice of all variadic arguments.
/// - [`ParsedArgs::pos`] – Retrieve a positional argument by its index.
#[derive(Debug, Clone)]
pub struct ParsedArgs {
    /// Values for options (arguments that take a value)
    values: HashMap<String, String>,
    /// Flags (true if present, false otherwise)
    flags: HashMap<String, bool>,
    /// Positional arguments (ordered)
    positional: Vec<String>,
    /// Variadic arguments (remaining arguments after positional)
    variadic: Vec<String>,
}

impl ParsedArgs {
    /// Retrieve the value of an option by name.
    pub fn get(&self, name: &str) -> Option<&String> {
        self.values.get(name)
    }
    
    /// Check if a flag was set.
    pub fn flag(&self, name: &str) -> bool {
        self.flags.get(name).copied().unwrap_or(false)
    }
    
    /// Get a slice of all positional arguments.
    pub fn positional(&self) -> &[String] {
        &self.positional
    }
    
    /// Get a slice of all variadic arguments.
    pub fn variadic(&self) -> &[String] {
        &self.variadic
    }
    
    /// Retrieve a positional argument by its index.
    pub fn pos(&self, index: usize) -> Option<&String> {
        self.positional.get(index)
    }
}

/// Represents a single CLI command in `luhcli`.
///  
/// Commands can have:
/// - Arguments (`Arg`)
/// - Subcommands (`Command`)
/// - A handler function executed when the command is invoked.
///
/// # Examples
///
/// ```ignore
/// use luhcli::{Command, Arg, ParsedArgs};
/// use luhtwin::LuhTwin;
///
/// let config_cmd = Command::new("config")
///     .about("edit the moth.json config (has all the general options)")
///     .arg(Arg::positional("action", 0)
///         .possible_values(["get", "set", "list"])
///         .help("action to perform"))
///     .arg(Arg::positional("key", 1)
///         .help("key to retrieve/set")
///         .when("get", vec![Arg::positional("key", 1)]))
///     .handler(|parsed: &ParsedArgs| {
///         LuhTwin::done()
///     });
///
/// let app_cmd = Command::new("app")
///     .about("start, stop, restart or list info about the app >.<");
///
/// let moth = Command::new("moth")
///     .about("your favourite music player <3")
///     .subcommand(config_cmd)
///     .subcommand(app_cmd);
/// 
/// // Print the help for the top-level command
/// moth.print_help("moth");
/// ```
///
/// This would output something like:
///
/// ```text
/// ╭─────────────────────────────────────────────────────────────────╮
/// │  moth                                                           
/// │  your favourite music player <3                                 
/// ╰─────────────────────────────────────────────────────────────────╯
///
/// commands:
///   config               edit the moth.json config (has all the general options)
///   app                  start, stop, restart or list info about the app >.<
///
/// for a subcommand:
///
/// ╭─────────────────────────────────────────────────────────────────╮
/// │  moth config                                                   
/// │  edit the moth.json config (has all the general options)       
/// ╰─────────────────────────────────────────────────────────────────╯
///
/// arguments:
///   <action> [get|set|list]                action to perform
///
///   when action = 'get':
///         <key>                            key to retrieve
///
///   when action = 'set':
///         <key>                            key to set
///         <value>                          value to set
/// ```
///
/// # Provided Methods
///
/// - [`Command::new`] – Create a new command with a name.
/// - [`Command::about`] – Set the about description.
/// - [`Command::usage`] – Set the usage string.
/// - [`Command::arg`] – Add an argument to the command.
/// - [`Command::subcommand`] – Add a subcommand.
/// - [`Command::handler`] – Set the handler function.
/// - [`Command::print_help`] – Print the help output to the console.
pub struct Command {
    name: String,
    about: String,
    usage: String,
    args: Vec<Arg>,
    subcommands: Vec<Command>,
    handler: Option<Box<dyn Fn(&ParsedArgs) -> LuhTwin<()>>>,
}

impl Command {
    /// Create a new command with a name.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            about: String::new(),
            usage: String::new(),
            args: Vec::new(),
            subcommands: Vec::new(),
            handler: None,
        }
    }
    
    /// Set the about description.
    pub fn about(mut self, about: impl Into<String>) -> Self {
        self.about = about.into();
        self
    }
    
    /// Set the usage string.
    pub fn usage(mut self, usage: impl Into<String>) -> Self {
        self.usage = usage.into();
        self
    }
    
    /// Add an argument to the command.
    pub fn arg(mut self, arg: Arg) -> Self {
        self.args.push(arg);
        self
    }
    
    /// Add a subcommand.
    pub fn subcommand(mut self, cmd: Command) -> Self {
        self.subcommands.push(cmd);
        self
    }
    
    /// Set the handler function.
    pub fn handler<F>(mut self, f: F) -> Self 
    where 
        F: Fn(&ParsedArgs) -> LuhTwin<()> + 'static 
    {
        self.handler = Some(Box::new(f));
        self
    }
    
    fn get_active_args(&self, parsed_positionals: &[String]) -> Vec<Arg> {
        let mut active_args = self.args.clone();
        
        for arg in &self.args {
            if let ArgType::Positional { index } = arg.arg_type {
                if let Some(value) = parsed_positionals.get(index) {
                    for chain in &arg.children {
                        if &chain.when_value == value {
                            for child in &chain.args {
                                let mut child_clone = child.clone();
                                if let ArgType::Positional { index: child_idx } = child_clone.arg_type {
                                    child_clone.arg_type = ArgType::Positional { 
                                        index: index + 1 + child_idx 
                                    };
                                }
                                active_args.push(child_clone);
                            }
                        }
                    }
                }
            }
        }
        
        active_args
    }
    
    fn parse(&self, args: &[String]) -> LuhTwin<ParsedArgs> {
        let mut values = HashMap::new();
        let mut flags = HashMap::new();
        let mut positional = Vec::new();
        let mut variadic = Vec::new();
        let mut seen_args = Vec::new();
        let mut i = 0;
        
        let mut positional_raw = Vec::new();
        
        let mut temp_i = 0;
        while temp_i < args.len() {
            let arg = &args[temp_i];
            if !arg.starts_with('-') {
                positional_raw.push(arg.clone());
            }
            temp_i += 1;
        }
        
        let active_args = self.get_active_args(&positional_raw);
        
        positional_raw.clear();
        
        while i < args.len() {
            let arg = &args[i];
            
            if arg.starts_with("--") {
                let key = arg.trim_start_matches("--");
                
                if let Some((k, v)) = key.split_once('=') {
                    if let Some(arg_def) = active_args.iter().find(|a| a.long.as_deref() == Some(k)) {
                        values.insert(arg_def.name.clone(), v.to_string());
                        seen_args.push(arg_def.name.clone());
                    } else {
                        return Err(at!("unknown option: --{}", k).into());
                    }
                } else if let Some(arg_def) = active_args.iter().find(|a| a.long.as_deref() == Some(key)) {
                    match arg_def.arg_type {
                        ArgType::Option => {
                            i += 1;
                            if i < args.len() {
                                values.insert(arg_def.name.clone(), args[i].clone());
                                seen_args.push(arg_def.name.clone());
                            } else {
                                return Err(at!("--{} requires a value", key).into());
                            }
                        }
                        ArgType::Flag => {
                            flags.insert(arg_def.name.clone(), true);
                            seen_args.push(arg_def.name.clone());
                        }
                        _ => return Err(at!("invalid argument type for --{}", key).into()),
                    }
                } else {
                    return Err(at!("unknown option: --{}", key).into());
                }
            } else if arg.starts_with('-') && arg.len() > 1 {
                let c = arg.chars().nth(1).unwrap();
                
                if let Some(arg_def) = active_args.iter().find(|a| a.short == Some(c)) {
                    match arg_def.arg_type {
                        ArgType::Option => {
                            i += 1;
                            if i < args.len() {
                                values.insert(arg_def.name.clone(), args[i].clone());
                                seen_args.push(arg_def.name.clone());
                            } else {
                                return Err(at!("-{} requires a value", c).into());
                            }
                        }
                        ArgType::Flag => {
                            flags.insert(arg_def.name.clone(), true);
                            seen_args.push(arg_def.name.clone());
                        }
                        _ => return Err(at!("invalid argument type for -{}", c).into()),
                    }
                } else {
                    return Err(at!("unknown option: -{}", c).into());
                }
            } else {
                positional_raw.push(arg.clone());
            }
            
            i += 1;
        }
        
        let positional_defs: Vec<_> = active_args.iter()
            .filter(|a| matches!(a.arg_type, ArgType::Positional { .. }))
            .collect();
        
        let variadic_def = active_args.iter()
            .find(|a| matches!(a.arg_type, ArgType::Variadic));
        
        let mut positional_sorted = positional_defs.clone();
        positional_sorted.sort_by_key(|a| {
            if let ArgType::Positional { index } = a.arg_type {
                index
            } else {
                usize::MAX
            }
        });
        
        for arg_def in positional_sorted.iter() {
            if let ArgType::Positional { index } = arg_def.arg_type {
                if let Some(value) = positional_raw.get(index) {
                    // Validate possible values
                    if !arg_def.possible_values.is_empty() && !arg_def.possible_values.contains(value) {
                        return Err(at!(
                            "invalid value '{}' for '{}'. possible values: {}",
                            value,
                            arg_def.name,
                            arg_def.possible_values.join(", ")
                        ).into());
                    }
                    
                    positional.push(value.clone());
                    values.insert(arg_def.name.clone(), value.clone());
                    seen_args.push(arg_def.name.clone());
                } else if arg_def.required {
                    if let Some(default) = &arg_def.default_value {
                        positional.push(default.clone());
                        values.insert(arg_def.name.clone(), default.clone());
                    } else {
                        return Err(at!("missing required positional argument: {}", arg_def.name).into());
                    }
                }
            }
        }
        
        if let Some(variadic_def) = variadic_def {
            let start_index = positional_sorted.len();
            variadic = positional_raw.get(start_index..).unwrap_or(&[]).to_vec();
            
            if !variadic.is_empty() {
                seen_args.push(variadic_def.name.clone());
            }
        }
        
        for arg_def in &active_args {
            if seen_args.contains(&arg_def.name) {
                for dep in &arg_def.depends_on {
                    if !seen_args.contains(dep) {
                        return Err(at!(
                            "'{}' requires '{}' to be specified",
                            arg_def.name,
                            dep
                        ).into());
                    }
                }
                
                for conflict in &arg_def.conflicts_with {
                    if seen_args.contains(conflict) {
                        return Err(at!(
                            "'{}' conflicts with '{}'",
                            arg_def.name,
                            conflict
                        ).into());
                    }
                }
            }
        }
        
        for arg_def in &active_args {
            if arg_def.required && !seen_args.contains(&arg_def.name) {
                if let Some(default) = &arg_def.default_value {
                    match arg_def.arg_type {
                        ArgType::Option => {
                            values.insert(arg_def.name.clone(), default.clone());
                        }
                        ArgType::Flag => {
                            flags.insert(arg_def.name.clone(), true);
                        }
                        _ => {}
                    }
                } else {
                    return Err(at!("required argument '{}' not provided", arg_def.name).into());
                }
            }
        }
        
        Ok(ParsedArgs { values, flags, positional, variadic })
    }
    
    /// Set the handler function.
    pub fn print_help(&self, full_path: &str) {
        use std::fmt::Write as _;

        let mut out = String::new();

        writeln!(out, "\n╭─────────────────────────────────────────────────────────────────╮").unwrap();
        writeln!(out, "│  {}  ", full_path).unwrap();
        writeln!(out, "│  {}  ", self.about).unwrap();
        writeln!(out, "╰─────────────────────────────────────────────────────────────────╯").unwrap();
        
        let positional_args: Vec<_> = self.args.iter()
            .filter(|a| matches!(a.arg_type, ArgType::Positional { .. } | ArgType::Variadic))
            .collect();
        
        if !positional_args.is_empty() {
            writeln!(out, "\narguments:").unwrap();
            for arg in positional_args {
                let mut arg_str = format!("  <{}>", arg.name);
                if matches!(arg.arg_type, ArgType::Variadic) {
                    arg_str = format!("  <{}>...", arg.name);
                }
                
                if !arg.possible_values.is_empty() {
                    arg_str.push_str(&format!(" [{}]", arg.possible_values.join("|")));
                }
                
                writeln!(out, "{:<40} {}", arg_str, arg.help).unwrap();
                
                if !arg.depends_on.is_empty() {
                    writeln!(out, "    depends on: {}", arg.depends_on.join(", ")).unwrap();
                }
                
                if !arg.children.is_empty() {
                    writeln!(out).unwrap();
                    for chain in &arg.children {
                        writeln!(out, "  when {} = '{}':", arg.name, chain.when_value).unwrap();
                        
                        for child in &chain.args {
                            let child_str = match child.arg_type {
                                ArgType::Positional { .. } => format!("    <{}>", child.name),
                                ArgType::Variadic => format!("    <{}>...", child.name),
                                _ => continue,
                            };
                            writeln!(out, "    {:<36} {}", child_str, child.help).unwrap();
                        }
                        
                        let child_options: Vec<_> = chain.args.iter()
                            .filter(|a| matches!(a.arg_type, ArgType::Flag | ArgType::Option))
                            .collect();
                        
                        if !child_options.is_empty() {
                            writeln!(out).unwrap();
                            writeln!(out, "    additional options:").unwrap();
                            for child in child_options {
                                let mut opt_str = String::from("      ");
                                
                                if let Some(s) = child.short {
                                    opt_str.push_str(&format!("-{}", s));
                                    if child.long.is_some() {
                                        opt_str.push_str(", ");
                                    }
                                }
                                
                                if let Some(l) = &child.long {
                                    opt_str.push_str(&format!("--{}", l));
                                }
                                
                                if matches!(child.arg_type, ArgType::Option) {
                                    opt_str.push_str(&format!(" <{}>", child.name));
                                }
                                
                                writeln!(out, "    {:<36} {}", opt_str, child.help).unwrap();
                            }
                        }
                        
                        writeln!(out).unwrap();
                    }
                }
            }
        }
        
        let option_args: Vec<_> = self.args.iter()
            .filter(|a| matches!(a.arg_type, ArgType::Flag | ArgType::Option))
            .collect();
        
        if !option_args.is_empty() {
            writeln!(out, "\noptions:").unwrap();
            for arg in option_args {
                let mut opt_str = String::from("  ");
                
                if let Some(s) = arg.short {
                    opt_str.push_str(&format!("-{}", s));
                    if arg.long.is_some() {
                        opt_str.push_str(", ");
                    }
                }
                
                if let Some(l) = &arg.long {
                    opt_str.push_str(&format!("--{}", l));
                }
                
                if matches!(arg.arg_type, ArgType::Option) {
                    opt_str.push_str(&format!(" <{}>", arg.name));
                }
                
                writeln!(out, "{:<30} {}", opt_str, arg.help).unwrap();
                
                if !arg.depends_on.is_empty() {
                    writeln!(out, "{:<30}   depends on: {}", "", arg.depends_on.join(", ")).unwrap();
                }
                
                if !arg.conflicts_with.is_empty() {
                    writeln!(out, "{:<30}   conflicts with: {}", "", arg.conflicts_with.join(", ")).unwrap();
                }
                
                if let Some(default) = &arg.default_value {
                    writeln!(out, "{:<30}   default: {}", "", default).unwrap();
                }
            }
        }
        
        if !self.subcommands.is_empty() {
            writeln!(out, "\ncommands:").unwrap();
            for sub in &self.subcommands {
                writeln!(out, "  {:<20} {}", sub.name, sub.about).unwrap();
            }
        }

        while out.ends_with('\n') || out.ends_with(' ') {
            out.pop();
        }

        println!("{}", out);
    }
}

/// Represents a complete CLI application built with `luhcli`.
///
/// `CliApp` wraps a root command and provides a convenient interface
/// to define global arguments, subcommands, and handle parsing and execution.
///
/// It is essentially the entry point of your CLI application.
///
/// # Examples
///
/// ```ignore
/// use luhcli::{CliApp, Arg, Command, ParsedArgs};
/// use luhtwin::LuhTwin;
///
/// // Define a subcommand
/// let hello_cmd = Command::new("hello")
///     .about("Say hello")
///     .arg(Arg::new("name")
///         .short('n')
///         .long("name")
///         .takes_value()
///         .help("Name of the person"))
///     .handler(|parsed: &ParsedArgs| {
///         if let Some(name) = parsed.get("name") {
///             println!("Hello, {}!", name);
///         } else {
///             println!("Hello, world!");
///         }
///         LuhTwin::done()
///     });
///
/// // Create the CLI app
/// let app = CliApp::new("greeter")
///     .about("A simple greeting app")
///     .subcommand(hello_cmd);
///
/// // Run the app (parses args from command line)
/// app.run()?;
/// ```
///
/// This would allow running in the terminal like:
/// ```text
/// $ greeter hello --name Alice
/// Hello, Alice!
/// ```
pub struct CliApp {
    root: Command,
}

impl CliApp {
    /// Create a new CLI application with a given name.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let app = CliApp::new("myapp");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            root: Command::new(name),
        }
    }
    
    /// Set the about/description of the CLI application.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let app = CliApp::new("myapp").about("This is my CLI application");
    /// ```
    pub fn about(mut self, about: impl Into<String>) -> Self {
        self.root = self.root.about(about);
        self
    }
    
    /// Add a global argument to the root command.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let app = CliApp::new("myapp")
    ///     .arg(Arg::new("verbose").short('v').help("Enable verbose output"));
    /// ```
    pub fn arg(mut self, arg: Arg) -> Self {
        self.root = self.root.arg(arg);
        self
    }
    
    /// Add a subcommand to the CLI application.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let sub = Command::new("start").about("Start the app");
    /// let app = CliApp::new("myapp").subcommand(sub);
    /// ```
    pub fn subcommand(mut self, cmd: Command) -> Self {
        self.root = self.root.subcommand(cmd);
        self
    }
    
    /// Run the CLI application, parsing command-line arguments from `std::env::args()`.
    ///
    /// This is the main entry point to execute the application.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let app = CliApp::new("myapp");
    /// app.run();
    /// ```
    pub fn run(self) -> LuhTwin<()> {
        let args: Vec<String> = std::env::args().skip(1).collect();
        self.run_with_args(&args)
    }

    /// Check if a string represents a help request.
    fn is_help(&self, thing: &str) -> bool {
        thing == "help" || thing == "--help" || thing == "-help"
    }
    
    fn run_with_args(&self, args: &[String]) -> LuhTwin<()> {
        if args.is_empty() || self.is_help(&args[0]) {
            self.root.print_help(&self.root.name);
            println!();
            return Ok(());
        }
        
        if let Some(subcmd) = self.root.subcommands.iter().find(|s| s.name == args[0]) {
            let subcmd_args = &args[1..];
            
            if subcmd_args.is_empty() || self.is_help(&subcmd_args[0]) {
                subcmd.print_help(&format!("{} {}", self.root.name, subcmd.name));
                println!("{}", subcmd.usage);
                return Ok(());
            }
            
            let parsed = subcmd.parse(subcmd_args)?;
            
            if let Some(handler) = &subcmd.handler {
                return handler(&parsed);
            } else {
                return Err(at!("no handler for command '{}'", subcmd.name).into());
            }
        }
        
        let parsed = self.root.parse(args)?;
        
        if let Some(handler) = &self.root.handler {
            handler(&parsed)
        } else {
            error!("unknown command: {}", args[0]);
            self.root.print_help(&self.root.name);
            std::process::exit(1);
        }
    }
}

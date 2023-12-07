use super::ServerConfig;
use arguments::{Argument, ArgumentError};
use command::{Command, CommandError};
use console::Console;
use std::{fmt::Display, io, str::FromStr};

pub fn start(config: ServerConfig) {
    let console = Console::new(config);
    console.start()
}

mod console {
    use super::*;

    #[derive(Debug)]
    pub struct Console {
        pub server_config: ServerConfig,
    }

    impl Console {
        pub fn new(server_config: ServerConfig) -> Self {
            Self { server_config }
        }

        pub fn start(&self) {
            eprintln!("\x1b[2J\x1b[1;4mChesstacean Console v1.0.0\x1b[0m\n\x1b[1mUse \"help\" for help\n\x1b[0m");
            loop {
                // Take Input
                eprint!(" > ");
                let mut buffer = String::new();
                let read = io::stdin().read_line(&mut buffer);

                // Run
                if let Ok(_) = read {
                    match self.run_command(Self::process(buffer)) {
                        None => break,
                        Some(msg) => eprintln!("{msg}"),
                    }
                } else if let Err(e) = read {
                    eprintln!("read_line error: {e}");
                }
            }

            eprintln!("\x1b[92;1mShutting Down!\x1b[0m");
        }

        fn run_command(&self, (cmd, args): (Option<String>, Vec<String>)) -> Option<String> {
            let command = Command::parse(cmd);
            match command {
                Ok(command) => {
                    let arguments = Argument::parse(args, &command);

                    // if let Ok(Command::Stop) = command {
                    //     return None;
                    // }

                    eprint!("\x1b[1m"); // Bold

                    // if let Err(e) = command {
                    //     eprintln!("{}", Self::error(&e));
                    // } else if let Ok(command) = command {
                    //     eprintln!("{}", command.msg(&self));
                    // }

                    eprintln!("\x1b[0m"); //Un-format

                    Some(Self::message(&"name", &"msg"))
                }
                Err(error) => Some(Self::error(&error)),
            }
        }

        fn process(input: String) -> (Option<String>, Vec<String>) {
            let input = input.trim_end_matches(&['\n', '\r']);
            let mut args = input.split_whitespace().map(|s| String::from(s));
            let cmd = args.next();
            let args: Vec<String> = args.collect();
            (cmd, args)
        }

        pub fn error(error: &impl Display) -> String {
            format!("\x1b[31m{error}")
        }

        pub fn message(name: &impl Display, msg: &impl Display) -> String {
            format!("\x1b[4m{name}\n\x1b[24m{msg}")
        }
    }
}

mod command {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub enum CommandError {
        Unknown(String),
        NoCommand,
    }

    impl Display for CommandError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let (name, context, location) = match self {
                Self::Unknown(cmd) => (
                    format!("Unknown Command"),
                    format!("{} does not exist", cmd),
                    format!("{}", cmd),
                ),
                Self::NoCommand => (
                    format!("No Command"),
                    format!("No command validly submitted"),
                    String::new(),
                ),
            };
            write!(f, "CommandError: {name}\n{context}\n > {location} << Here")
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Command {
        Help,
        Stop,
        Config,
    }

    impl Command {
        // const HELP: [(&'static str, &'static str); 4] = [
        //     ("config", "Get the server's current configuration"),
        //     (
        //         "help <cmd?>",
        //         "Lists all commands, or gets information about a specific command",
        //     ),
        //     ("stop", "Stop the server"),
        //     ("Commands", "help <cmd?> \nconfig \nstop"),
        // ];
        pub fn parse(cmd: Option<String>) -> Result<Self, CommandError> {
            if let Some(cmd) = cmd {
                match &cmd[..] {
                    "stop" => Ok(Self::Stop),
                    "help" => Ok(Self::Help),
                    "config" => Ok(Self::Config),
                    _ => Err(CommandError::Unknown(cmd)),
                }
            } else {
                Err(CommandError::NoCommand)
            }
        }
        pub fn name(&self) -> String {
            match self {
                Self::Config => format!("config"),
                Self::Help => format!("help"),
                Self::Stop => format!("ctop"),
            }
        }
        // pub fn msg(&self, console: &Console) -> String {
        //     match self {
        //         Self::Help(args) => {
        //             let (name, msg) = match args {
        //                 HelpArguments::Config => Command::HELP[0],
        //                 HelpArguments::Help => Command::HELP[1],
        //                 HelpArguments::Stop => Command::HELP[2],
        //                 HelpArguments::All => Command::HELP[3],
        //             };
        //             Console::message(&name, &msg)
        //         }
        //         Self::Config => Console::message(&"Server Config", &console.server_config),
        //         _ => String::new(),
        //     }
        // }
    }

    impl Display for Command {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let (name, msg) = match self {
                Self::Config => ("config", "Get the server's current configuration"),
                Self::Help => (
                    "help <cmd?>",
                    "Lists all commands, or gets information about a specific command",
                ),
                Self::Stop => ("stop", "Stop the server"),
            };
            write!(f, "{}", Console::message(&name, &msg))
        }
    }

    impl FromStr for Command {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {}
        }
    }

    // #[cfg(test)]
    // mod tests {
    //     use super::*;

    //     #[test]
    //     fn blank_cmd() {
    //         let err = ConsoleError::from(CommandError(
    //             String::from("No Command"),
    //             String::from("No command validly submitted"),
    //             String::new(),
    //         ));

    //         let cmd: Result<Command, ConsoleError> = String::from("").parse();
    //         match_helper(&cmd, &err);

    //         let cmd: Result<Command, ConsoleError> = String::from("       ").parse();
    //         match_helper(&cmd, &err);

    //         let cmd: Result<Command, ConsoleError> = String::new().parse();
    //         match_helper(&cmd, &err);
    //     }

    //     #[test]
    //     fn valid_cmd() {
    //         let cmd: Command = String::from("stop").parse().unwrap();
    //         assert_eq!(cmd, Command::Stop);

    //         let cmd: Command = String::from("config").parse().unwrap();
    //         assert_eq!(cmd, Command::Config);

    //         let cmd: Command = String::from("help").parse().unwrap();
    //         assert_eq!(cmd, Command::Help(HelpArguments::All));

    //         let cmd: Command = String::from("help stop").parse().unwrap();
    //         assert_eq!(cmd, Command::Help(HelpArguments::Stop));

    //         let cmd: Command = String::from("help config").parse().unwrap();
    //         assert_eq!(cmd, Command::Help(HelpArguments::Config));

    //         let cmd: Command = String::from("help help").parse().unwrap();
    //         assert_eq!(cmd, Command::Help(HelpArguments::Help));
    //     }

    //     #[test]
    //     fn invalid_cmd() {
    //         let err = ConsoleError::from(CommandError(
    //             String::from("Unknown Command"),
    //             format!("{} does not exist", "foo"),
    //             String::from("foo"),
    //         ));

    //         let cmd: Result<Command, ConsoleError> = String::from("foo bar").parse();
    //         match_helper(&cmd, &err);
    //     }

    //     // TODO: Test for excess args on stop and config

    //     #[test]
    //     fn invalid_help() {
    //         let cmd: Result<Command, ConsoleError> = String::from("help foo").parse();
    //         match_helper(
    //             &cmd,
    //             &ConsoleError::from(
    //                 ArgumentError(
    //                     String::from("Invalid"),
    //                     format!(
    //                         "{} is not a valid name of a command. Try running 'help' without arguments to see all commands",
    //                         "foo"
    //                     ),
    //                     format!("help {}", "foo")
    //                 )
    //             )
    //         );

    //         let cmd: Result<Command, ConsoleError> = String::from("help foo bar").parse();
    //         match_helper(
    //             &cmd,
    //             &ConsoleError::from(ArgumentError(
    //                 String::from("Too Many"),
    //                 format!("{} arguments were found, only 1 was expected", 2),
    //                 format!("help {0} {1}", "foo", "bar"),
    //             )),
    //         );

    //         let cmd: Result<Command, ConsoleError> = String::from("help config foo bar baz").parse();
    //         match_helper(
    //             &cmd,
    //             &ConsoleError::from(ArgumentError(
    //                 String::from("Too Many"),
    //                 format!("{} arguments were found, only 1 was expected", 4),
    //                 format!("help {0} {1}", "config", "foo"),
    //             )),
    //         );
    //     }

    //     fn match_helper(x: &Result<Command, ConsoleError>, eq: &ConsoleError) {
    //         match x {
    //             Ok(_) => panic!("Should be a ConsoleError"),
    //             Err(e) => assert_eq!(eq, e),
    //         }
    //     }
    // }
}

mod arguments {
    use self::ArgOption::{None, Optional, Required};
    use super::*;

    enum ArgOption {
        None,
        Required(Vec<ArgDef>),
        Optional(Vec<ArgDef>),
    }

    // impl ArgOption {
    //     fn max_len(&self) -> usize {
    //         match self {
    //             None => 0,
    //             Required(defs) => defs.iter().map(|def| def.max_len()).max().unwrap_or(0) + 1,
    //             Optional(defs) => defs.iter().map(|def| def.max_len()).max().unwrap_or(0) + 1,
    //         }
    //     }
    // }

    enum ArgDef {
        Command(ArgOption),
    }

    // impl ArgDef {
    //     fn max_len(&self) -> usize {
    //         match self {
    //             ArgDef::Command(opt) => opt.max_len(),
    //         }
    //     }
    // }

    pub enum Argument {
        None,
        Command(Command, Option<Box<Argument>>),
    }

    impl Argument {
        pub fn parse(args: Vec<String>, cmd: &Command) -> Result<Vec<Argument>, ArgumentError> {
            let definition = Self::cmd_args(&cmd);
            // Self::check_length(&args, &definition, &cmd)?;
            Self::parse_iter(args.iter(), &definition, cmd.name(), (args.len(), 0));
            Err(ArgumentError::Invalid {
                info: format!(""),
                location: format!(""),
            })
        }

        fn cmd_args(cmd: &Command) -> ArgOption {
            match cmd {
                Command::Config => None,
                Command::Help => Optional(vec![ArgDef::Command(None)]),
                Command::Stop => None,
            }
        }

        fn parse_iter<'a, I>(
            mut iter: I,
            opt: &ArgOption,
            mut location: String,
            mut count: (usize, usize),
        ) -> Result<Argument, ArgumentError>
        where
            I: Iterator<Item = &'a String>,
        {
            let arg = iter.next();
            match arg {
                Option::None => {
                    if let Required(def) = opt {
                        Err(ArgumentError::NotEnough {
                            found: count.0,
                            expect: count.1 + 1,
                            location: location + "_",
                        })
                    } else {
                        Ok(Argument::None)
                    }
                }
                Option::Some(arg) => {
                    location.push_str(&format!(" {arg}"));
                    match opt {
                        None => Err(ArgumentError::TooMany {
                            found: count.0,
                            expect: count.1,
                            location,
                        }),
                        Optional(def) => {
                            let parsed = Self::parse_single(arg, &def, &location)?;
                        }
                        Required(def) => {}
                    }
                }
            }
        }

        // fn check_length(args: &Vec<String>, def: &ArgOption, cmd: &Command) -> Result<(), ArgumentError> {
        //     let max = def.max_len();
        //     if args.len() > max {
        //         let loc = args[..(max + 1)]
        //             .iter()
        //             .map(|x| format!("{x}"))
        //             .reduce(|a, b| format!("{a} {b}"))
        //             .unwrap_or(format!(""));
        //         Err(ArgumentError::TooMany {
        //             found: args.len(),
        //             expect: max,
        //             location: format!("{0} {loc}", cmd.name()),
        //         })
        //     } else {
        //         Ok(())
        //     }

        //     Ok(())
        // }

        fn parse_single<'a>(
            arg: &String,
            def_vec: &'a Vec<ArgDef>,
            location: &String,
        ) -> Result<&'a ArgOption, ArgumentError> {
            for def in def_vec {
                match def {
                    ArgDef::Command(opt) => {
                        let parsed: Result<Command, _> = arg.parse();
                        return Ok(opt);
                    }
                }
            }
            Err(ArgumentError::Invalid {
                location: format!("{}", location),
            })
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum ArgumentError {
        TooMany {
            found: usize,
            expect: usize,
            location: String,
        },
        NotEnough {
            found: usize,
            expect: usize,
            location: String,
        },
        Invalid {
            location: String,
        },
    }

    fn cmpr1(s: &usize) -> &str {
        if s == &1 {
            "s"
        } else {
            ""
        }
    }

    impl Display for ArgumentError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let (name, context, location) = match self {
                Self::TooMany {
                    found,
                    expect,
                    location,
                } => (
                    format!("Too Many"),
                    format!(
                        "{found} argument{0} found, only {expect} argument{1} expected",
                        cmpr1(found),
                        cmpr1(expect)
                    ),
                    format!("{location}"),
                ),
                Self::NotEnough {
                    found,
                    expect,
                    location,
                } => (
                    format!("Not Enough"),
                    format!(
                        "{found} argument{0} found, {expect} argument{1} expected",
                        cmpr1(found),
                        cmpr1(expect)
                    ),
                    format!("{location}"),
                ),
                Self::Invalid { location } => (
                    format!("Invalid"),
                    format!("Argument is invalid"),
                    format!("{location}"),
                ),
            };
            write!(f, "ArgumentError: {name}\n{context}\n > {location} << Here")
        }
    }

    // #[derive(Debug, PartialEq)]
    // pub enum HelpArguments {
    //     All,
    //     Help,
    //     Stop,
    //     Config,
    // }

    // impl Argument for HelpArguments {
    //     type This = Self;

    //     fn parse(args: Vec<&str>) -> Result<Self, ArgumentError> {
    //         if args.len() > 1 {
    //             let mut iter = args.iter();
    //             return Err(ArgumentError(
    //                 String::from("Too Many"),
    //                 format!("{} arguments were found, only 1 was expected", args.len()),
    //                 format!(
    //                     "help {0} {1}",
    //                     iter.next().unwrap_or(&"None"),
    //                     iter.next().unwrap_or(&"None")
    //                 ),
    //             ));
    //         }
    //         let arg = args.iter().next();
    //         if let Some(arg) = arg {
    //             arg.parse()
    //         } else {
    //             Ok(HelpArguments::All)
    //         }
    //     }
    // }

    // impl FromStr for HelpArguments {
    //     type Err = ArgumentError;
    //     fn from_str(s: &str) -> Result<Self, Self::Err> {
    //         match s {
    //             "stop" => Ok(Self::Stop),
    //             "help" => Ok(Self::Help),
    //             "config" => Ok(Self::Config),
    //             _ =>
    //                 Err(
    //                     ArgumentError(
    //                         String::from("Invalid"),
    //                         format!("{} is not a valid name of a command. Try running 'help' without arguments to see all commands", s),
    //                         format!("help {}", s)
    //                     )
    //                 ),
    //         }
    //     }
    // }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn too_many_args() {
            let result = Argument::parse(vec![format!("config"), format!("foo")], &Command::Help);
            if let Err(e) = result {
                assert_eq!(
                    e,
                    ArgumentError::TooMany {
                        found: 2,
                        expect: 1,
                        location: format!("help config foo")
                    }
                )
            } else {
                panic!("Should be Err")
            }
        }
    }
    //     #[test]
    //     fn no_args() {
    //         let args = HelpArguments::parse(vec![]).unwrap();
    //         assert_eq!(args, HelpArguments::All);
    //     }

    //     #[test]
    //     fn valid_args() {
    //         let args = HelpArguments::parse(vec!["help"]).unwrap();
    //         assert_eq!(args, HelpArguments::Help);

    //         let args = HelpArguments::parse(vec!["config"]).unwrap();
    //         assert_eq!(args, HelpArguments::Config);

    //         let args = HelpArguments::parse(vec!["stop"]).unwrap();
    //         assert_eq!(args, HelpArguments::Stop);
    //     }

    //     #[test]
    //     fn too_many_args() {
    //         match_helper(
    //             HelpArguments::parse(vec!["help", "foo"]),
    //             ArgumentError(
    //                 String::from("Too Many"),
    //                 format!("{} arguments were found, only 1 was expected", 2),
    //                 format!("help {0} {1}", "help", "foo"),
    //             ),
    //         );

    //         match_helper(
    //             HelpArguments::parse(vec!["help", "foo", "bar", "baz"]),
    //             ArgumentError(
    //                 String::from("Too Many"),
    //                 format!("{} arguments were found, only 1 was expected", 4),
    //                 format!("help {0} {1}", "help", "foo"),
    //             ),
    //         )
    //     }

    //     #[test]
    //     fn invalid_arg() {
    //         match_helper(
    //             HelpArguments::parse(vec!["hel"]),
    //             ArgumentError(
    //                 String::from("Invalid"),
    //                 format!(
    //                     "{} is not a valid name of a command. Try running 'help' without arguments to see all commands",
    //                     "hel"
    //                 ),
    //                 format!("help {}", "hel")
    //             )
    //         )
    //     }

    //     fn match_helper(x: Result<HelpArguments, ArgumentError>, eq: ArgumentError) {
    //         match x {
    //             Ok(_) => panic!("Should be an ArgumentError"),
    //             Err(e) => {
    //                 assert_eq!(e, eq);
    //             }
    //         }
    //     }
    // }
}

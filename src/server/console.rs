use super::ServerConfig;
use arguements::{Arguement, ArguementError, HelpArguements};
use command::{Command, CommandError};
use console::{Console, ConsoleError};
use std::{fmt::Display, io, str::FromStr};

pub fn start(config: ServerConfig) {
    let console = Console::new(config);
    console.start()
}

mod console {
    use super::*;

    #[derive(Debug, PartialEq)]
    pub enum ConsoleError {
        CommandError(CommandError),
        ArguementError(ArguementError),
    }

    impl Display for ConsoleError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "{}",
                match self {
                    Self::CommandError(a) => format!("{}", a),
                    Self::ArguementError(b) => format!("{}", b),
                }
            )
        }
    }

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
                eprint!(" > ");
                let mut buffer = String::new();
                let read = io::stdin().read_line(&mut buffer);
                if let Ok(_) = read {
                    if self.run_command(buffer.parse()).is_none() {
                        break;
                    }
                } else if let Err(e) = read {
                    eprintln!("read_line error: {e}");
                }
            }
        }

        fn run_command(&self, command: Result<Command, ConsoleError>) -> Option<()> {
            if let Ok(Command::Stop) = command {
                eprintln!("\x1b[92;1mShutting Down!\x1b[0m");
                return None;
            }

            eprint!("\x1b[1m"); // Bold

            if let Err(e) = command {
                eprintln!("{}", Self::error(&e));
            } else if let Ok(command) = command {
                eprintln!("{}", command.msg(&self));
            }

            eprintln!("\x1b[0m"); //Un-format

            Some(())
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
    pub struct CommandError(pub String, pub String, pub String);

    impl Display for CommandError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "CommandError: {0}\n{1}\n > {2} << Here", self.0, self.1, self.2)
        }
    }

    impl From<CommandError> for ConsoleError {
        fn from(value: CommandError) -> Self {
            ConsoleError::CommandError(value)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Command {
        Help(HelpArguements),
        Stop,
        Config,
    }

    impl Command {
        const HELP: [(&'static str, &'static str); 4] = [
            ("config", "Get the server's current configuration"),
            (
                "help <cmd?>",
                "Lists all commands, or gets information about a specific command",
            ),
            ("stop", "Stop the server"),
            ("Commands", "help <cmd?> \nconfig \nstop"),
        ];
        pub fn msg(&self, console: &Console) -> String {
            match self {
                Self::Help(args) => {
                    let (name, msg) = match args {
                        HelpArguements::Config => Command::HELP[0],
                        HelpArguements::Help => Command::HELP[1],
                        HelpArguements::Stop => Command::HELP[2],
                        HelpArguements::All => Command::HELP[3],
                    };
                    Console::message(&name, &msg)
                }
                Self::Config => Console::message(&"Server Config", &console.server_config),
                _ => String::new(),
            }
        }
    }

    impl FromStr for Command {
        type Err = ConsoleError;

        fn from_str(s: &str) -> Result<Command, Self::Err> {
            let s = s.trim_end_matches(&['\n', '\r']);
            let mut args = s.split_whitespace();
            let cmd = args.next();
            let args: Vec<&str> = args.collect();
            if let Some(cmd) = cmd {
                match cmd {
                    "stop" => Ok(Self::Stop),
                    "help" => Ok(Self::Help(HelpArguements::parse(args)?)),
                    "config" => Ok(Self::Config),
                    _ => Err(ConsoleError::from(CommandError(
                        String::from("Unknown Command"),
                        format!("{} does not exist", cmd),
                        String::from(cmd),
                    ))),
                }
            } else {
                Err(ConsoleError::from(CommandError(
                    String::from("No Command"),
                    String::from("No command validly submitted"),
                    String::new(),
                )))
            }
        }
    }

    impl Display for Command {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "")
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn blank_cmd() {
            let err = ConsoleError::from(CommandError(
                String::from("No Command"),
                String::from("No command validly submitted"),
                String::new(),
            ));

            let cmd: Result<Command, ConsoleError> = String::from("").parse();
            match_helper(&cmd, &err);

            let cmd: Result<Command, ConsoleError> = String::from("       ").parse();
            match_helper(&cmd, &err);

            let cmd: Result<Command, ConsoleError> = String::new().parse();
            match_helper(&cmd, &err);
        }

        #[test]
        fn valid_cmd() {
            let cmd: Command = String::from("stop").parse().unwrap();
            assert_eq!(cmd, Command::Stop);

            let cmd: Command = String::from("config").parse().unwrap();
            assert_eq!(cmd, Command::Config);

            let cmd: Command = String::from("help").parse().unwrap();
            assert_eq!(cmd, Command::Help(HelpArguements::All));

            let cmd: Command = String::from("help stop").parse().unwrap();
            assert_eq!(cmd, Command::Help(HelpArguements::Stop));

            let cmd: Command = String::from("help config").parse().unwrap();
            assert_eq!(cmd, Command::Help(HelpArguements::Config));

            let cmd: Command = String::from("help help").parse().unwrap();
            assert_eq!(cmd, Command::Help(HelpArguements::Help));
        }

        #[test]
        fn invalid_cmd() {
            let err = ConsoleError::from(CommandError(
                String::from("Unknown Command"),
                format!("{} does not exist", "foo"),
                String::from("foo"),
            ));

            let cmd: Result<Command, ConsoleError> = String::from("foo bar").parse();
            match_helper(&cmd, &err);
        }

        // TODO: Test for excess args on stop and config

        #[test]
        fn invalid_help() {
            let cmd: Result<Command, ConsoleError> = String::from("help foo").parse();
            match_helper(
                &cmd,
                &ConsoleError::from(
                    ArguementError(
                        String::from("Invalid"),
                        format!(
                            "{} is not a valid name of a command. Try running 'help' without arguements to see all commands",
                            "foo"
                        ),
                        format!("help {}", "foo")
                    )
                )
            );

            let cmd: Result<Command, ConsoleError> = String::from("help foo bar").parse();
            match_helper(
                &cmd,
                &ConsoleError::from(ArguementError(
                    String::from("Too Many"),
                    format!("{} arguements were found, only 1 was expected", 2),
                    format!("help {0} {1}", "foo", "bar"),
                )),
            );

            let cmd: Result<Command, ConsoleError> = String::from("help config foo bar baz").parse();
            match_helper(
                &cmd,
                &ConsoleError::from(ArguementError(
                    String::from("Too Many"),
                    format!("{} arguements were found, only 1 was expected", 4),
                    format!("help {0} {1}", "config", "foo"),
                )),
            );
        }

        fn match_helper(x: &Result<Command, ConsoleError>, eq: &ConsoleError) {
            match x {
                Ok(_) => panic!("Should be a ConsoleError"),
                Err(e) => assert_eq!(eq, e),
            }
        }
    }
}

mod arguements {
    use super::*;

    pub trait Arguement {
        type This;

        fn parse(args: Vec<&str>) -> Result<Self::This, ArguementError>;
    }

    #[derive(Debug, PartialEq)]
    pub struct ArguementError(pub String, pub String, pub String);

    impl Display for ArguementError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ArguementError: {0}\n{1}\n > {2} << Here", self.0, self.1, self.2)
        }
    }

    impl From<ArguementError> for ConsoleError {
        fn from(value: ArguementError) -> Self {
            ConsoleError::ArguementError(value)
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum HelpArguements {
        All,
        Help,
        Stop,
        Config,
    }

    impl Arguement for HelpArguements {
        type This = Self;

        fn parse(args: Vec<&str>) -> Result<Self, ArguementError> {
            if args.len() > 1 {
                let mut iter = args.iter();
                return Err(ArguementError(
                    String::from("Too Many"),
                    format!("{} arguements were found, only 1 was expected", args.len()),
                    format!(
                        "help {0} {1}",
                        iter.next().unwrap_or(&"None"),
                        iter.next().unwrap_or(&"None")
                    ),
                ));
            }
            let arg = args.iter().next();
            if let Some(arg) = arg {
                arg.parse()
            } else {
                Ok(HelpArguements::All)
            }
        }
    }

    impl FromStr for HelpArguements {
        type Err = ArguementError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "stop" => Ok(Self::Stop),
                "help" => Ok(Self::Help),
                "config" => Ok(Self::Config),
                _ =>
                    Err(
                        ArguementError(
                            String::from("Invalid"),
                            format!("{} is not a valid name of a command. Try running 'help' without arguements to see all commands", s),
                            format!("help {}", s)
                        )
                    ),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn no_args() {
            let args = HelpArguements::parse(vec![]).unwrap();
            assert_eq!(args, HelpArguements::All);
        }

        #[test]
        fn valid_args() {
            let args = HelpArguements::parse(vec!["help"]).unwrap();
            assert_eq!(args, HelpArguements::Help);

            let args = HelpArguements::parse(vec!["config"]).unwrap();
            assert_eq!(args, HelpArguements::Config);

            let args = HelpArguements::parse(vec!["stop"]).unwrap();
            assert_eq!(args, HelpArguements::Stop);
        }

        #[test]
        fn too_many_args() {
            match_helper(
                HelpArguements::parse(vec!["help", "foo"]),
                ArguementError(
                    String::from("Too Many"),
                    format!("{} arguements were found, only 1 was expected", 2),
                    format!("help {0} {1}", "help", "foo"),
                ),
            );

            match_helper(
                HelpArguements::parse(vec!["help", "foo", "bar", "baz"]),
                ArguementError(
                    String::from("Too Many"),
                    format!("{} arguements were found, only 1 was expected", 4),
                    format!("help {0} {1}", "help", "foo"),
                ),
            )
        }

        #[test]
        fn invalid_arg() {
            match_helper(
                HelpArguements::parse(vec!["hel"]),
                ArguementError(
                    String::from("Invalid"),
                    format!(
                        "{} is not a valid name of a command. Try running 'help' without arguements to see all commands",
                        "hel"
                    ),
                    format!("help {}", "hel")
                )
            )
        }

        fn match_helper(x: Result<HelpArguements, ArguementError>, eq: ArguementError) {
            match x {
                Ok(_) => panic!("Should be an ArguementError"),
                Err(e) => {
                    assert_eq!(e, eq);
                }
            }
        }
    }
}

use super::ServerConfig;
use arguments::Argument;
use command::Command;
use console::Console;
use std::{cell::RefCell, fmt::Display, io, slice::Iter, str::FromStr, vec};

pub fn start(config: ServerConfig) {
    Console::new(config).start();
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
            eprintln!("\x1b[2J\x1b[1;4mChesstacean Console v1.1.0\x1b[0m\n\x1b[1mUse \"help\" for help\n\x1b[0m");
            loop {
                // Take Input
                eprint!(" > ");
                let mut buffer = String::new();
                let read = io::stdin().read_line(&mut buffer);

                // Run
                if let Ok(_) = read {
                    match self.process_command(Self::process(buffer)) {
                        None => {
                            break;
                        }
                        Some(msg) => eprintln!("{msg}\x1b[0m\n"),
                    }
                } else if let Err(e) = read {
                    eprintln!("read_line error: {e}");
                }
            }

            eprintln!("\x1b[92;1mShutting Down!\x1b[0m");
        }

        fn process_command(&self, (cmd, args): (Option<String>, Vec<String>)) -> Option<String> {
            eprint!("\x1b[1m"); // Bold
            match Command::parse(cmd) {
                Ok(command) => match Argument::parse(args, &command) {
                    Ok(arguments) => self.run(command, arguments),
                    Err(error) => Some(Self::error(&error)),
                },
                Err(error) => Some(Self::error(&error)),
            }
        }

        fn run(&self, cmd: Command, args: Argument) -> Option<String> {
            match cmd {
                Command::Stop => None,
                Command::Config => Some(Self::message(&"Server Config", &self.server_config)),
                Command::Help => Some(if let Argument::Command(search, _) = args {
                    Self::message(&"Help", &search)
                } else {
                    Self::message(&"Help", &"help <cmd?> \nconfig \nstop")
                }),
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
                Self::Stop => format!("stop"),
            }
        }
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
            write!(f, "{}: {}", name, msg)
        }
    }

    impl FromStr for Command {
        type Err = ();
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match &s[..] {
                "stop" => Ok(Self::Stop),
                "help" => Ok(Self::Help),
                "config" => Ok(Self::Config),
                _ => Err(()),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        #[allow(unused_imports)]
        use super::*;
    }
}

mod arguments {
    use self::ArgOption::{None, Optional, Required, RequiredChain};
    use super::*;

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    enum ArgOption {
        None,
        Required(Box<ArgDef>),
        RequiredChain(Vec<ArgDef>),
        Optional(Vec<ArgDef>),
    }

    impl ArgOption {
        fn len(&self, length: u16) -> (u16, String) {
            match self {
                Required(def) => def.len(length + 1),
                RequiredChain(_) => (length + 1, format!("+")),
                _ => (length, format!("")),
            }
        }
    }

    #[derive(Debug, Clone)]
    struct ArgDef {
        opt: ArgOption,
        def: Def,
    }

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    enum Def {
        Command,
        Number,
    }

    impl ArgDef {
        fn new(def: Def, opt: ArgOption) -> Self {
            Self { opt, def }
        }
        #[allow(dead_code)]
        fn new_boxed(def: Def, opt: ArgOption) -> Box<Self> {
            Box::new(Self::new(def, opt))
        }
        fn len(&self, length: u16) -> (u16, String) {
            self.opt.len(length)
        }
    }

    struct ArgParser<'c> {
        found: usize,
        expect: RefCell<u16>,
        iter: RefCell<Iter<'c, String>>,
        location: RefCell<String>,
    }

    impl<'c> ArgParser<'c> {
        fn new(iter: Iter<'c, String>, name: String, found: usize) -> Self {
            Self {
                found,
                expect: RefCell::new(0),
                iter: RefCell::new(iter),
                location: RefCell::new(name),
            }
        }
        fn parse_iter<'a>(&self, definition: ArgOption) -> Result<Argument, ArgumentError> {
            let arg = self.iter.borrow_mut().next();
            match arg {
                Option::None => {
                    if let Required(_) = &definition {
                        self.location.borrow();
                        Err(ArgumentError::NotEnough {
                            found: self.found,
                            expect: definition.len(*self.expect.borrow()),
                            location: self.location.borrow().to_string(),
                        })
                    } else if let RequiredChain(_) = definition {
                        self.location.borrow();
                        Err(ArgumentError::NotEnough {
                            found: self.found,
                            expect: (*self.expect.borrow() + 1, format!("+")),
                            location: self.location.borrow().to_string(),
                        })
                    } else {
                        Ok(Argument::None)
                    }
                }
                Option::Some(arg) => {
                    self.location.borrow_mut().push_str(&format!(" {arg}"));
                    match definition {
                        None => Err(ArgumentError::TooMany {
                            found: self.found,
                            expect: *self.expect.borrow(),
                            location: self.location.borrow().to_string(),
                        }),
                        Required(def) => self.parse_arg(&arg, vec![*def]),
                        RequiredChain(def_list) => self.parse_arg(&arg, def_list),
                        Optional(def_list) => self.parse_arg(&arg, def_list),
                    }
                }
            }
        }

        fn parse_arg(&self, arg: &String, def_vec: Vec<ArgDef>) -> Result<Argument, ArgumentError> {
            for def in def_vec {
                match def.def {
                    Def::Command => {
                        let parsed = arg.parse::<Command>();
                        if let Ok(cmd) = parsed {
                            *self.expect.borrow_mut() += 1;
                            return Ok(Argument::Command(cmd, Box::new(self.parse_iter(def.opt)?)));
                        }
                    }
                    Def::Number => {
                        let parsed = arg.parse::<i32>();
                        if let Ok(cmd) = parsed {
                            *self.expect.borrow_mut() += 1;
                            return Ok(Argument::Number(cmd, Box::new(self.parse_iter(def.opt)?)));
                        }
                    }
                }
            }
            Err(ArgumentError::Invalid {
                location: self.location.borrow().to_string(),
            })
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum Argument {
        None,
        Command(Command, Box<Argument>),
        Number(i32, Box<Argument>),
    }

    impl Argument {
        pub fn parse(args: Vec<String>, cmd: &Command) -> Result<Argument, ArgumentError> {
            let definition = Self::cmd_args(&cmd);
            ArgParser::new(args.iter(), cmd.name(), args.len()).parse_iter(definition)
        }

        fn cmd_args(cmd: &Command) -> ArgOption {
            match cmd {
                Command::Config => None,
                Command::Help => Optional(vec![ArgDef::new(Def::Command, None)]),
                Command::Stop => None,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum ArgumentError {
        TooMany {
            found: usize,
            expect: u16,
            location: String,
        },
        NotEnough {
            found: usize,
            expect: (u16, String),
            location: String,
        },
        Invalid {
            location: String,
        },
    }

    fn cmpr1(s: &usize) -> &str {
        if s == &1 {
            ""
        } else {
            "s"
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
                        cmpr1(&usize::from(expect.clone()))
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
                        "{found} argument{0} found, {1}{2} argument{3} expected",
                        cmpr1(found),
                        expect.0,
                        expect.1,
                        cmpr1(&usize::from(expect.0.clone()))
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

    #[cfg(test)]
    mod tests {
        use super::*;

        fn example_def() -> ArgOption {
            let br1 = None;
            let br1 = Required(ArgDef::new_boxed(Def::Number, br1));
            let br1 = Required(ArgDef::new_boxed(Def::Number, br1));
            let br1 = Required(ArgDef::new_boxed(Def::Number, br1));

            let br2 = None;
            let br2 = Optional(vec![ArgDef::new(Def::Number, br2)]);
            let br2 = Optional(vec![ArgDef::new(Def::Command, br2)]);

            let br2 = RequiredChain(vec![ArgDef::new(Def::Command, br2), ArgDef::new(Def::Number, None)]);

            let br1 = Optional(vec![ArgDef::new(Def::Command, br1), ArgDef::new(Def::Number, br2)]);

            let br1 = Required(ArgDef::new_boxed(Def::Number, br1));

            br1
        }

        fn test_example_def(args: Vec<&str>) -> Result<Argument, ArgumentError> {
            ArgParser::new(
                args.iter().map(|s| s.to_string()).collect::<Vec<String>>().iter(),
                format!("test"),
                args.len(),
            )
            .parse_iter(example_def())
        }

        fn should_err(res: Result<Argument, ArgumentError>) -> ArgumentError {
            match res {
                Ok(a) => panic!("Should be an ArguementError\nContext: {a:?}"),
                Err(e) => e,
            }
        }

        #[test]
        fn no_args() {
            let result = Argument::parse(vec![], &Command::Help).unwrap();
            assert_eq!(result, Argument::None);

            let result = Argument::parse(vec![], &Command::Config).unwrap();
            assert_eq!(result, Argument::None);

            let result = Argument::parse(vec![], &Command::Stop).unwrap();
            assert_eq!(result, Argument::None);

            let result = should_err(test_example_def(vec![]));
            assert_eq!(
                result,
                ArgumentError::NotEnough {
                    found: 0,
                    expect: (1, "".to_string()),
                    location: "test".to_string(),
                }
            )
        }

        #[test]
        fn too_many_args() {
            let result = should_err(Argument::parse(vec![format!("config"), format!("foo")], &Command::Help));
            assert_eq!(
                result,
                ArgumentError::TooMany {
                    found: 2,
                    expect: 1,
                    location: "help config foo".to_string(),
                }
            );

            let result = should_err(Argument::parse(vec![format!("foo")], &Command::Stop));
            assert_eq!(
                result,
                ArgumentError::TooMany {
                    found: 1,
                    expect: 0,
                    location: "stop foo".to_string(),
                }
            );

            let result = should_err(Argument::parse(vec![format!("foo")], &Command::Config));
            assert_eq!(
                result,
                ArgumentError::TooMany {
                    found: 1,
                    expect: 0,
                    location: "config foo".to_string(),
                }
            );

            let result = should_err(test_example_def(vec!["1234", "config", "1234", "1234", "1234", "foo"]));
            assert_eq!(
                result,
                ArgumentError::TooMany {
                    found: 6,
                    expect: 5,
                    location: "test 1234 config 1234 1234 1234 foo".to_string(),
                }
            );

            let result = should_err(test_example_def(vec!["1234", "56", "78", "foo", "bar"]));
            assert_eq!(
                result,
                ArgumentError::TooMany {
                    found: 5,
                    expect: 3,
                    location: "test 1234 56 78 foo".to_string(),
                }
            );

            let result = should_err(test_example_def(vec![
                "1234", "56", "config", "help", "78", "foo", "bar", "baz",
            ]));
            assert_eq!(
                result,
                ArgumentError::TooMany {
                    found: 8,
                    expect: 5,
                    location: "test 1234 56 config help 78 foo".to_string(),
                }
            );
        }

        #[test]
        fn not_enough_args() {
            let result = should_err(test_example_def(vec!["1234", "config"]));
            assert_eq!(
                result,
                ArgumentError::NotEnough {
                    found: 2,
                    expect: (5, "".to_string()),
                    location: "test 1234 config".to_string(),
                }
            );

            let result = should_err(test_example_def(vec!["1234", "config", "56"]));
            assert_eq!(
                result,
                ArgumentError::NotEnough {
                    found: 3,
                    expect: (5, "".to_string()),
                    location: "test 1234 config 56".to_string(),
                }
            );

            let result = should_err(test_example_def(vec!["1234", "config", "56", "78"]));
            assert_eq!(
                result,
                ArgumentError::NotEnough {
                    found: 4,
                    expect: (5, "".to_string()),
                    location: "test 1234 config 56 78".to_string(),
                }
            );

            let result = should_err(test_example_def(vec!["1234", "56"]));
            assert_eq!(
                result,
                ArgumentError::NotEnough {
                    found: 2,
                    expect: (3, "+".to_string()),
                    location: "test 1234 56".to_string(),
                }
            );
        }

        #[test]
        fn invalid_args() {
            let result = should_err(Argument::parse(vec![format!("foo")], &Command::Help));
            assert_eq!(
                result,
                ArgumentError::Invalid {
                    location: "help foo".to_string()
                }
            );

            let result = should_err(Argument::parse(vec![format!("foo"), format!("bar")], &Command::Help));
            assert_eq!(
                result,
                ArgumentError::Invalid {
                    location: "help foo".to_string()
                }
            );

            let result = should_err(test_example_def(vec!["1.1"]));
            assert_eq!(
                result,
                ArgumentError::Invalid {
                    location: "test 1.1".to_string(),
                }
            );
            let result = should_err(test_example_def(vec!["config"]));
            assert_eq!(
                result,
                ArgumentError::Invalid {
                    location: "test config".to_string(),
                }
            );

            let result = should_err(test_example_def(vec!["1234", "1.1"]));
            assert_eq!(
                result,
                ArgumentError::Invalid {
                    location: "test 1234 1.1".to_string(),
                }
            );
            let result = should_err(test_example_def(vec!["1234", "foo"]));
            assert_eq!(
                result,
                ArgumentError::Invalid {
                    location: "test 1234 foo".to_string(),
                }
            );
        }

        #[test]
        fn valid_args() {
            let result = Argument::parse(vec![], &Command::Help).unwrap();
            assert_eq!(result, Argument::None);
            let result = Argument::parse(vec![format!("help")], &Command::Help).unwrap();
            assert_eq!(result, Argument::Command(Command::Help, Box::new(Argument::None)));
            let result = Argument::parse(vec![format!("config")], &Command::Help).unwrap();
            assert_eq!(result, Argument::Command(Command::Config, Box::new(Argument::None)));
            let result = Argument::parse(vec![format!("stop")], &Command::Help).unwrap();
            assert_eq!(result, Argument::Command(Command::Stop, Box::new(Argument::None)));

            let result = Argument::parse(vec![], &Command::Config).unwrap();
            assert_eq!(result, Argument::None);

            let result = Argument::parse(vec![], &Command::Stop).unwrap();
            assert_eq!(result, Argument::None);

            let result = test_example_def(vec!["1234"]).unwrap();
            assert_eq!(result, Argument::Number(1234, Box::new(Argument::None)));
            let result = test_example_def(vec!["1234", "config", "12", "34", "56"]).unwrap();
            assert_eq!(
                result,
                Argument::Number(
                    1234,
                    Box::new(Argument::Command(
                        Command::Config,
                        Box::new(Argument::Number(
                            12,
                            Box::new(Argument::Number(
                                34,
                                Box::new(Argument::Number(56, Box::new(Argument::None)))
                            ))
                        ))
                    ))
                )
            );
            let result = test_example_def(vec!["12", "34", "56"]).unwrap();
            assert_eq!(
                result,
                Argument::Number(
                    12,
                    Box::new(Argument::Number(
                        34,
                        Box::new(Argument::Number(56, Box::new(Argument::None)))
                    ))
                )
            );
            let result = test_example_def(vec!["12", "34", "stop"]).unwrap();
            assert_eq!(
                result,
                Argument::Number(
                    12,
                    Box::new(Argument::Number(
                        34,
                        Box::new(Argument::Command(Command::Stop, Box::new(Argument::None)))
                    ))
                )
            );
            let result = test_example_def(vec!["12", "34", "stop", "config"]).unwrap();
            assert_eq!(
                result,
                Argument::Number(
                    12,
                    Box::new(Argument::Number(
                        34,
                        Box::new(Argument::Command(
                            Command::Stop,
                            Box::new(Argument::Command(Command::Config, Box::new(Argument::None)))
                        ))
                    ))
                )
            );
            let result = test_example_def(vec!["12", "34", "stop", "config", "56"]).unwrap();
            assert_eq!(
                result,
                Argument::Number(
                    12,
                    Box::new(Argument::Number(
                        34,
                        Box::new(Argument::Command(
                            Command::Stop,
                            Box::new(Argument::Command(
                                Command::Config,
                                Box::new(Argument::Number(56, Box::new(Argument::None)))
                            ))
                        ))
                    ))
                )
            );
        }
    }
}

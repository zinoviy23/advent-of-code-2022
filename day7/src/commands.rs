use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum Command {
    Cd(CdArg),
    Ls,
}

#[derive(Debug, Clone)]
pub enum CdArg {
    Parent,
    Root,
    Dir(String),
}

#[derive(Debug, Clone)]
pub enum LsOutput {
    Dir(String),
    File(String, u32),
}

#[derive(Debug, Clone)]
pub enum Input {
    Command(Command),
    LsOutput(LsOutput),
}

#[derive(Debug, Clone)]
pub struct CommandParseError(String);

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = s
            .strip_prefix("cd ")
            .map(|arg| {
                let arg = match arg.trim() {
                    ".." => CdArg::Parent,
                    "/" => CdArg::Root,
                    other => CdArg::Dir(other.to_string()),
                };
                Command::Cd(arg)
            })
            .or_else(|| Some(s).filter(|s| s.trim() == "ls").map(|_| Command::Ls));

        command.ok_or_else(|| CommandParseError(format!("Unknown command: {}", s)))
    }
}

impl FromStr for LsOutput {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(dir_name) = s.strip_prefix("dir ") {
            Ok(LsOutput::Dir(dir_name.to_string()))
        } else if let Some((size, name)) = s.split_once(" ") {
            let size: u32 = size.parse().map_err(|err| {
                CommandParseError(format!("Cannot parse output '{}' because {}", s, err))
            })?;
            Ok(LsOutput::File(name.to_string(), size))
        } else {
            Err(CommandParseError(format!("Unknown output format: {}", s)))
        }
    }
}

impl FromStr for Input {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(cmd) = s.strip_prefix("$ ") {
            cmd.parse::<Command>().map(|cmd| Input::Command(cmd))
        } else {
            s.parse::<LsOutput>().map(|output| Input::LsOutput(output))
        }
    }
}

//  Copyright © 2021 github.com/razn-v
//
//  This file is part of Wisp.
//
//  Wisp is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  Wisp is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with Wisp.  If not, see <https://www.gnu.org/licenses/>.

use std::fmt;
use std::result::Result;

use async_trait::async_trait;

use crate::controllers::app::App;
use crate::controllers::command_parser::ParseError::{IncorrectSyntax, NotFound};

#[derive(Debug)]
pub enum ParseError {
    NotFound,
    IncorrectSyntax,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::NotFound => write!(f, "Command/flag not found"),
            ParseError::IncorrectSyntax => write!(f, "Incorrect syntax"),
        }
    }
}

pub struct Flag {
    name: String,
    has_arg: bool,  // True if the flag is followed by an argument (ex. -o my_file)
}

impl Flag {
    pub fn new(name: &str, has_arg: bool) -> Self {
        Flag{ name: name.to_string(), has_arg }
    }
}

pub type Flags = Vec<(Flag, String)>;

pub fn get_flag_arg(flags: &Flags, name: &str) -> Option<String> {
    for flag in flags {
        if flag.0.name == name.to_string() {
            return Some(flag.1.clone())
        }
    }
    None
}

pub type Command = Box<dyn CommandTrait + Send + Sync>;
pub type Args = Vec<String>;

#[async_trait]
pub trait CommandTrait {
    // Return the minimum number of args needed
    fn min_args(&self) -> usize;
    fn flags(&self) -> Vec<Flag>;
    async fn exec(&self, args: Args, flags: Flags, app: &App);
}

pub struct CommandParser;

impl CommandParser {
    // Parse an input of args.
    // The first element must be a command name, otherwise an error is thrown (command not found).
    pub fn parse<'a>(&self, app: &'a App, args: Vec<String>) -> Result<(&'a Command, Args, Flags), ParseError> {
        if args.is_empty() {
            return Err(NotFound);
        }

        let (command_name, args2) = (&args[0], args[1..].to_vec());

        match app.options.get(command_name) {
            Some(cmd) => {
                if args2.len() >= cmd.min_args() {
                    let mut flags = Vec::<(Flag, String)>::new();

                    for flag in cmd.flags() {
                        if !args2.contains(&flag.name) {
                            continue;
                        }

                        if flag.has_arg {
                            let flag_pos = args.iter()
                                .position(|e| e == &flag.name)
                                .unwrap();

                            match args.get(flag_pos + 1) {
                                None => return Err(IncorrectSyntax), // Argument missing
                                Some(flag_arg) => flags.push((flag, flag_arg.clone())),
                            }
                        } else {
                            flags.push((flag, String::new()))
                        }
                    }

                    return Ok((cmd, args2, flags));
                }
                Err(IncorrectSyntax)
            }
            None => Err(NotFound)
        }
    }
}
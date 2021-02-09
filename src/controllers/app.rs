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

use std::collections::HashMap;

use crate::controllers::api::API;
use crate::controllers::command_parser::Command;
use crate::controllers::config::Config;

pub struct App {
    pub config: Config,
    pub options: HashMap<String, Command>,
    pub apis: Vec<API>,
}

impl App {
    pub fn new() -> Self {
        Self {
            config: Default::default(),
            options: Default::default(),
            apis: Vec::new(),
        }
    }

    pub fn set_config(&mut self, config: Config) {
        self.config = config;
    }

    pub fn add_option(&mut self, option: &str, command: Command) -> &mut Self {
        self.options.insert(option.to_owned(), command);
        self
    }

    pub fn add_api(&mut self, api: API) -> &mut Self {
        self.apis.push(api);
        self
    }
}
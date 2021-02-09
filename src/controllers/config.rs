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
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

use crate::controllers::api::APISource;
use crate::controllers::app::App;

pub struct Config {
    dir: PathBuf,
    pub apis: HashMap<APISource, String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            dir: dirs::home_dir()
                .expect("Could not get home directory!")
                .join(".wisp.cfg"),
            apis: HashMap::new(),
        }
    }

    pub fn parse_config(&mut self, app: &App) {
        if !self.dir.exists() {
            self.create_config_file(app)
        }

        let config_file = File::open(&self.dir).unwrap();
        let reader = BufReader::new(config_file);

        for (i, line) in reader.lines().enumerate() {
            let line = line.unwrap().replace(" ", "");
            let split = line.splitn(2, "=").collect::<Vec<&str>>();
            let (key, value) = (split[0], split[1]);

            match APISource::from_str(key, app) {
                Some(api) => {
                    self.apis.insert(api, value.to_string())
                }
                None => panic!("{} option is incorrect at line {} (api not found)", key, i + 1),
            };
        }
    }

    fn create_config_file(&self, app: &App) {
        let mut file = File::create(&self.dir)
            .expect("Could not create config file!");

        // Write each option into config file
        for api in &app.apis {
            let option = api.get_type().to_string().to_uppercase();
            file.write_all(format!("{}=\n", option).as_bytes())
                .expect(&format!("Could not write option {} into config file!", option));
        }

        println!("Created config file at: {}", self.dir.display())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            dir: Default::default(),
            apis: Default::default(),
        }
    }
}
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

use std::fs::File;
use std::io::copy;

use async_trait::async_trait;

use crate::controllers::app::App;
use crate::controllers::command_parser::{Args, Command, CommandTrait, Flag, Flags, get_flag_arg};

pub struct DownloadCommand;

impl DownloadCommand {
    pub fn new() -> Command {
        Box::new(Self {})
    }
}

#[async_trait]
impl CommandTrait for DownloadCommand {
    fn min_args(&self) -> usize {
        1
    }

    fn flags(&self) -> Vec<Flag> {
        vec![
            Flag::new("-ht", true),
            Flag::new("-o", true),
        ]
    }

    async fn exec(&self, args: Args, flags: Flags, app: &App) {
        let client = reqwest::Client::new();

        for (api_src, api_key) in &app.config.apis {
            for api in &app.apis {
                if &api.get_type() == api_src {
                    let hash_type = get_flag_arg(&flags, "-ht")
                        .unwrap_or(String::from("sha256"));

                    let res = api.get_request(&client, &api_key, &args[0], &hash_type)
                        .send().await
                        .unwrap();

                    if res.status() == reqwest::StatusCode::OK {
                        println!("Malware found from {} api", api_src.to_string());

                        let file_name = get_flag_arg(&flags, "-o")
                            .unwrap_or(String::from("out.bin"));

                        let mut out = File::create(file_name)
                            .expect("Failed to create file!");

                        copy(&mut res.bytes().await.unwrap().as_ref(), &mut out)
                            .expect("Failed to copy content!");

                        return;
                    }
                }
            }
        }

        println!("Malware not found!");
    }
}

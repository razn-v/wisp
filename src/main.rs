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

use std::env;

use crate::apis::koodous::KoodousAPI;
use crate::apis::malshare::MalshareAPI;
use crate::apis::polyswarm::PolyswarmAPI;
use crate::apis::unpacme::UnpacmeAPI;
use crate::commands::download::DownloadCommand;
use crate::controllers::app::App;
use crate::controllers::command_parser::CommandParser;
use crate::controllers::config::Config;

mod controllers;
mod commands;
mod apis;

#[tokio::main]
async fn main() {
    let command_parser = CommandParser {};
    let mut args: Vec<String> = env::args().collect();

    // Remove the first arg, which is the name of the program
    args = args.drain(1..).collect();

    let mut app = App::new();
    app.add_option("dl", DownloadCommand::new())
        .add_api(MalshareAPI::new())
        .add_api(UnpacmeAPI::new())
        .add_api(PolyswarmAPI::new())
        .add_api(KoodousAPI::new());

    let mut config = Config::new();
    config.parse_config(&app);

    app.set_config(config);

    match command_parser.parse(&app, args) {
        Ok(res) => res.0.exec(res.1, res.2, &app).await,
        Err(err) => println!("{}", err),
    };
}
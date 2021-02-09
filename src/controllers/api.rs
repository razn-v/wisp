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

use core::fmt;
use std::fmt::Formatter;

use crate::controllers::app::App;

pub type API = Box<dyn APITrait + Send + Sync>;

pub trait APITrait {
    fn get_type(&self) -> APISource;
    fn get_request(&self, client: &reqwest::Client, api_key: &str, hash: &str, hash_type: &str)
                   -> reqwest::RequestBuilder;
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum APISource {
    Malshare,
    Unpacme,
    Polyswarm,
    Koodous,
}

impl fmt::Display for APISource {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl APISource {
    pub fn from_str(s: &str, app: &App) -> Option<Self> {
        for api in &app.apis {
            if s == api.get_type().to_string().to_uppercase() {
                return Some(api.get_type());
            }
        }
        None
    }
}
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

use crate::controllers::api::{API, APISource, APITrait};
use crate::controllers::api::APISource::Polyswarm;

pub struct PolyswarmAPI;

impl PolyswarmAPI {
    pub fn new() -> API {
        Box::new(Self {})
    }
}

impl APITrait for PolyswarmAPI {
    fn get_type(&self) -> APISource {
        Polyswarm
    }

    fn get_request(&self, client: &reqwest::Client, api_key: &str, hash: &str, hash_type: &str)
                   -> reqwest::RequestBuilder
    {
        client.get(&format!("https://api.polyswarm.network/v2/download/{}/{}", hash_type, hash))
            .header("Authorization", api_key)
    }
}
// Copyright © 2014, Canal TP and/or its affiliates. All rights reserved.
//
// LICENCE: This program is free software; you can redistribute it
// and/or modify it under the terms of the GNU Affero General Public
// License as published by the Free Software Foundation, either
// version 3 of the License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public
// License along with this program. If not, see
// <http://www.gnu.org/licenses/>.

use index;

#[deriving(Decodable, Encodable)]
pub struct Bano {
    pub id: String,
    pub nb: String,
    pub street: String,
    pub zip: String,
    pub city: String,
    pub src: String,
    pub lat: f64,
    pub lon: f64,
}
impl Bano {
    pub fn insee(&self) -> &str {
        assert!(self.id.len() >= 5);
        self.id[..5]
    }
    pub fn fantoir(&self) -> &str {
        assert!(self.id.len() >= 10);
        self.id[..10]
    }
    pub fn into_addr(self) -> index::Addr {
        let street_name = format!("{}, {} {}", self.street, self.zip, self.city);
        let addr_name = format!("{} {}", self.nb, street_name);
        let street_id = format!("street:{}", self.fantoir().to_string());
        let admin = index::Admin {
            id: format!("admin:{}", self.insee()),
            level: 8,
            name: self.city,
            zip_code: self.zip,
            weight: 1,
        };
        let street = index::Street {
            id: street_id,
            street_name: self.street,
            name: street_name,
            administrative_region: admin,
            weight: 1,
        };
        index::Addr {
            id: format!("addr:{};{}", self.lat, self.lon),
            house_number: self.nb,
            street: street,
            name: addr_name,
            coord: index::Coord { lat: self.lat, lon: self.lon },
            weight: 1,
        }
    }
}

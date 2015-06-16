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

extern crate csv;
extern crate rustc_serialize;
extern crate curl;
extern crate docopt;
extern crate iron;
extern crate urlencoded;

mod index;
mod bano;

use std::path::Path;
use iron::prelude::*;
use iron::headers::ContentType;
use iron::status;
use urlencoded::UrlEncodedQuery;
use rustc_serialize::json::Json;

fn index_bano(files: &[String]) {
    println!("purge and create Munin...");
    index::purge_and_create_munin().unwrap();
    println!("Munin purged and created.");

    for f in files.iter() {
        println!("importing {}...", f);
        let mut rdr = csv::Reader::from_file(&Path::new(&f)).unwrap().has_headers(false);
        let iter = rdr.decode().map(|r| { let b: bano::Bano = r.unwrap(); b.into_addr() });
        let nb = index::index(iter).unwrap();
        println!("importing {}: {} addresses added.", f, nb);
    }
}

fn query(q: &str) -> Result<curl::http::Response, curl::ErrCode> {
    use rustc_serialize::json::Json::String;
    let query = format!(include_str!("../json/query.json"), query=String(q.to_string()));
    println!("{}", query);
    let resp = curl::http::handle()
        .post("http://localhost:9200/munin/_search?pretty", &query)
        .exec();
    resp
}

fn geocode(lon: f64, lat: f64) -> Result<(), curl::ErrCode> {
    let query = format!(include_str!("../json/geocode.json"), lon=lon, lat=lat);
    println!("{}", query);
    let r = try! {
        curl::http::handle().post("http://localhost:9200/munin/addr/_search?pretty&size=1", &query)
            .exec()
    };
    println!("{}", r);
    Ok(())
}

fn make_obj(v: Vec<(&'static str, Json)>) -> Json {
    use rustc_serialize::json::Json::Object;
    Object(v.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}

fn make_feature(json: &Json) -> Json {
    use rustc_serialize::json::Json::*;

    make_obj(vec![
        ("properties", make_obj(vec![
            ("label", json.find("name")
                          .map(|j| j.clone())
                          .unwrap_or(Null)),
            ("housenumber", json.find("house_number")
                                .map(|j| j.clone())
                                .unwrap_or(Null)),
            ("street", json.find("street")
                           .and_then(|s| s.find("street_name"))
                           .map(|j| j.clone())
                           .unwrap_or(Null)),
            ("postcode", json.find("street")
                             .and_then(|s| s.find("administrative_region"))
                             .and_then(|s| s.find("zip_code"))
                             .map(|j| j.clone())
                             .unwrap_or(Null)),
            ("city", json.find("street")
                         .and_then(|s| s.find("administrative_region"))
                         .and_then(|s| s.find("name"))
                         .map(|j| j.clone())
                         .unwrap_or(Null)),
            ("country", String("France".to_string()))
            ])),
        ("type", String("Feature".to_string())),
        ("geometry", make_obj(vec![
            ("type", String("Point".to_string())),
            ("coordinates", Array(vec![
                json.find("coord")
                    .and_then(|j| j.find("lon"))
                    .map(|j| j.clone())
                    .unwrap_or(Null),
                json.find("coord")
                    .and_then(|j| j.find("lat"))
                    .map(|j| j.clone())
                    .unwrap_or(Null)
                ]))
            ])),
        ])
}

fn handle_query(req: &mut Request) -> IronResult<Response> {
    use rustc_serialize::json::Json::*;

    let map = req.get_ref::<UrlEncodedQuery>().unwrap();
    let q = map.get("q").and_then(|v| v.get(0)).unwrap();
    let es = query(q).unwrap();
    let es = Json::from_str(std::str::from_utf8(es.get_body()).unwrap()).unwrap();
    let sources: Vec<_> = es.find("hits")
        .and_then(|hs| hs.find("hits"))
        .and_then(|hs| hs.as_array())
        .map(|hs| hs.iter()
             .filter_map(|h| h.find("_source"))
             .map(|s| make_feature(s)))
        .unwrap()
        .collect();
    let json = make_obj(vec![
        ("type", String("FeatureCollection".to_string())),
        ("version", String("0.1.0".to_string())),
        ("query", String(q.clone())),
        ("feature", Array(sources))
    ]);

    let mut resp = Response::with((status::Ok, format!("{}", json.pretty())));
    resp.headers.set(ContentType("application/json".parse().unwrap()));
    Ok(resp)
}

fn runserver() {
    Iron::new(handle_query).http("0.0.0.0:3000").unwrap();
}

#[derive(RustcDecodable, Debug)]
struct Args {
    cmd_index: bool,
    cmd_query: bool,
    cmd_geocode: bool,
    cmd_runserver: bool,
    arg_bano_files: Vec<String>,
    arg_query: String,
    arg_lon: Option<f64>,
    arg_lat: Option<f64>
}
static USAGE: &'static str = "
Usage:
    munin index <bano-files>...
    munin query <query>
    munin geocode [--] <lon> <lat>
    munin runserver
";

fn main() {
    let args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.cmd_index {
        index_bano(&args.arg_bano_files);
    } else if args.cmd_query {
        println!("{}", query(&args.arg_query).unwrap());
    } else if args.cmd_geocode {
        geocode(args.arg_lon.unwrap(), args.arg_lat.unwrap()).unwrap();
    } else if args.cmd_runserver {
        runserver();
    } else {
        unreachable!();
    }
}

#![feature(slicing_syntax, phase, tuple_indexing)]

extern crate csv;
extern crate serialize;
extern crate curl;
extern crate docopt;
#[phase(plugin)] extern crate docopt_macros;

mod index;
mod bano;

fn index_bano(files: &[String]) {
    println!("purge and create Munin...");
    index::purge_and_create_munin().unwrap();
    println!("Munin purged and created.");

    for f in files.iter() {
        println!("importing {}...", f);
        let mut rdr = csv::Reader::from_file(&Path::new(f[])).has_headers(false);
        let iter = rdr.decode().map(|r| { let b: bano::Bano = r.unwrap(); b.into_addr() });
        let nb = index::index(iter).unwrap();
        println!("importing {}: {} addresses added.", f, nb);
    }
}

fn to_json_string(s: &str) -> String {
    use serialize::json;

    let mut w = std::io::MemWriter::with_capacity(s.len() + 2);

    // as `s` is a valid utf8 string, and we use MemWriter, these
    // unwrap can't fail
    json::escape_bytes(&mut w, s.as_bytes()).unwrap();
    String::from_utf8(w.unwrap()).unwrap()
}

fn query(q: &str) -> Result<(), curl::ErrCode> {
    let query = format!(include_str!("../json/query.json"), query=to_json_string(q)[]);
    println!("{}", query);
    let r = try! {
        curl::http::handle().post("http://localhost:9200/munin/_search?pretty", query[])
            .exec()
    };
    println!("{}", r);
    Ok(())
}

fn geocode(lon: f64, lat: f64) -> Result<(), curl::ErrCode> {
    let query = format!(include_str!("../json/geocode.json"), lon=lon, lat=lat);
    println!("{}", query);
    let r = try! {
        curl::http::handle().post("http://localhost:9200/munin/addr/_search?pretty&size=1", query[])
            .exec()
    };
    println!("{}", r);
    Ok(())
}

docopt!(Args deriving Show, "
Usage:
    munin index <bano-files>...
    munin query <query>
    munin geocode [--] <lon> <lat>
", arg_lon: Option<f64>, arg_lat: Option<f64>)

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    if args.cmd_index {
        index_bano(args.arg_bano_files[]);
    } else if args.cmd_query {
        query(args.arg_query[]).unwrap();
    } else if args.cmd_geocode {
        geocode(args.arg_lon.unwrap(), args.arg_lat.unwrap()).unwrap();
    } else {
        unreachable!();
    }

}

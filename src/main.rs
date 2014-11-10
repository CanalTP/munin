#![feature(slicing_syntax, phase)]

extern crate csv;
extern crate serialize;
extern crate curl;
extern crate docopt;
#[phase(plugin)] extern crate docopt_macros;

mod index;

#[deriving(Decodable, Encodable)]
struct Bano {
    id: String,
    nb: String,
    street: String,
    zip: String,
    city: String,
    src: String,
    lat: f64,
    lon: f64,
}
impl Bano {
    fn insee(&self) -> &str {
        assert!(self.id.len() >= 5);
        self.id[..5]
    }
    fn into_addr(self) -> index::Addr {
        let addr = format!("{} {}, {} {}", self.nb, self.street, self.zip, self.city);
        let admin = index::Admin {
            id: format!("admin:{}", self.insee()),
            level: 8,
            name: self.city,
            zip_code: self.zip
        };
        index::Addr {
            id: format!("addr:{};{}", self.lat, self.lon),
            house_number: self.nb,
            name: self.street,
            administrative_region: admin,
            coord: index::Coord { lat: self.lat, lon: self.lon },
            addr: addr,
        }
    }
}

fn index_bano(files: &[String]) {
    println!("purge and create Munin...");
    index::purge_and_create_munin().unwrap();
    println!("Munin purged and created.");

    for f in files.iter() {
        println!("importing {}...", f);
        let mut rdr = csv::Reader::from_file(&Path::new(f[])).has_headers(false);
        let iter = rdr.decode().map(|r| { let b: Bano = r.unwrap(); b.into_addr() });
        let nb = index::index(iter).unwrap();
        println!("importing {}: {} addresses added.", f, nb);
    }
}

docopt!(Args deriving Show, "
Usage:
    munin index <bano-files>...
    munin query <query>
")

fn query(q: &str) -> Result<(), curl::ErrCode> {
    let query = format!(include_str!("../json/query.json"), query=q);
    println!("{}", query);
    let r = try! {
        curl::http::handle().post("http://localhost:9200/munin/addr/_search?pretty", &*query)
            .exec()
    };
    println!("{}", r);
    Ok(())
}

fn main() {
    let args: Args = Args::docopt().decode().unwrap_or_else(|e| e.exit());

    if args.cmd_index {
        index_bano(args.arg_bano_files[]);
    } else if args.cmd_query {
        query(&*args.arg_query).unwrap();
    } else {
        unreachable!();
    }

}

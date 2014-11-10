use serialize::json;
use curl;

#[deriving(Decodable, Encodable)]
pub struct Coord { pub lat: f64, pub lon: f64 }

#[deriving(Decodable, Encodable)]
pub struct Admin {
    pub id: String,
    pub level: uint,
    pub name: String,
    pub zip_code: String,
}

#[deriving(Decodable, Encodable)]
pub struct Addr {
    pub id: String,
    pub house_number: String,
    pub name: String,
    pub administrative_region: Admin,
    pub coord: Coord,
    pub addr: String,
}

fn push_bulk(s: &mut String, addr: &Addr) {
    s.push_str("{index: {}}\n");    
    s.push_str(json::encode(addr)[]);
    s.push('\n');
}

type CurlResult = Result<curl::http::Response, curl::ErrCode>;

pub fn purge_and_create_munin() -> Result<(), curl::ErrCode> {
    use serialize::json::Json;

    // first, we must delete with its own handle the old munin
    try!(curl::http::handle().delete("http://localhost:9200/munin").exec());

    let analysis = include_str!("../json/settings.json");
    assert!(from_str::<Json>(analysis).is_some());
    let res = try!(curl::http::handle().put("http://localhost:9200/munin", analysis).exec());
    assert!(res.get_code() == 200, format!("Error adding analysis: {}", res));

    Ok(())
}

pub fn index<I: Iterator<Addr>>(mut iter: I) -> Result<uint, curl::ErrCode> {
    let mut handle = curl::http::handle();
    let mut nb = 0;
    let mut chunk = String::new();
    loop {
        chunk.clear();
        let addr = match iter.next() { Some(a) => a, None => break };
        push_bulk(&mut chunk, &addr);
        nb += 1;
        for addr in iter.by_ref().take(1000) {
            push_bulk(&mut chunk, &addr);
            nb += 1;
        }
        let res = try!(handle.post("http://localhost:9200/munin/addr/_bulk", chunk[]).exec());
        assert!(res.get_code() != 201, format!("result of bulk insert is not 200: {}", res));
    }
    Ok(nb)
}
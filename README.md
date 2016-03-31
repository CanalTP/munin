munin
=====

Autocomplete prototype. See http://fr.wikipedia.org/wiki/Hugin_et_Munin

To build, you must first install rust:
```shell
curl -s https://static.rust-lang.org/rustup.sh | sudo sh
```
and then build munin:
```shell
cd munin
cargo build --release
cd ..
```

Now, you must create the elasticsearch database. First, run elasticsearch:
```shell
curl -O https://download.elasticsearch.org/elasticsearch/elasticsearch/elasticsearch-1.4.0.tar.gz
tar xzvf elasticsearch-*.tar.gz
cd elasticsearch-*/
./bin/elasticsearch&
cd ..
```
and then index bano:
```shell
curl -O http://bano.openstreetmap.fr/data/old_2014/BANO-France-20140901-csv.zip
unzip BANO-France-20140901-csv.zip
./munin/target/release/munin index bano-data*/bano-*.csv
```

Now, you can run query and reverse geoloc
```shell
time ./munin/target/release/munin query "20 bou poni pa"
time ./munin/target/release/munin geocode 2.39137 48.82964
```

It gives:
```
{
    "query": {
        "filtered": {
            "query": {
                "bool": {
                    "should": [
                        {
                            "term": {
                                "_type": {
                                    "value": "addr",
                                    "boost": 1000
                                }
                            }
                        },
                        {
                            "match": {
                                "name.prefix": {
                                    "query": "20 bou poni pa",
                                    "boost": 100
                                }
                            }
                        },
                        {
                            "match": {
                                "name.ngram": {
                                    "query": "20 bou poni pa",
                                    "boost": 1
                                }
                            }
                        },
                        {
                            "function_score": {
                                "query": { "match_all": { } },
                                "field_value_factor": {
                                    "field": "weight",
                                    "modifier": "log1p",
                                    "factor": 1
                                },
                                "boost_mode": "multiply",
                                "boost": 30
                            }
                        }
                    ]
                }
            },
            "filter": {
                "bool": {
                    "should": [
                        { "missing": { "field": "house_number" } },
                        {
                            "query": {
                                "match": { "house_number": "20 bou poni pa" }
                            }
                        }
                    ],
                    "must": [
                        {
                            "query": {
                                "match": {
                                    "name.ngram": {
                                        "query": "20 bou poni pa",
                                        "minimum_should_match": "50%"
                                    }
                                }
                            }
                        }
                    ]
                }
            }
        }
    }
}

Response {200, content-length: 4958, content-type: application/json; charset=UTF-8, {
  "took" : 66,
  "timed_out" : false,
  "_shards" : {
    "total" : 16,
    "successful" : 16,
    "failed" : 0
  },
  "hits" : {
    "total" : 91220,
    "max_score" : 5.7747,
    "hits" : [ {
      "_index" : "munin",
      "_type" : "addr",
      "_id" : "OG0gGu91Spmn9FVIDw1GgQ",
      "_score" : 5.7747,
      "_source":{"id":"addr:48.829647;2.391372","house_number":"20","street":{"id":"street:751127597M","street_name":"Boulevard Poniatowski","name":"Boulevard Poniatowski, 75012 Paris","administrative_region":{"id":"admin:75112","level":8,"name":"Paris","zip_code":"75012","weight":1},"weight":1},"name":"20 Boulevard Poniatowski, 75012 Paris","coord":{"lat":48.829647,"lon":2.391372},"weight":1}
    }, {
      "_index" : "munin",
      "_type" : "street",
      "_id" : "vhy1tpckQ4WOYpkcAfaqlA",
      "_score" : 3.7323742,
      "_source":{"id":"street:751127597M","street_name":"Boulevard Poniatowski","name":"Boulevard Poniatowski, 75012 Paris","administrative_region":{"id":"admin:75112","level":8,"name":"Paris","zip_code":"75012","weight":1},"weight":90}
    }, {
      "_index" : "munin",
      "_type" : "addr",
      "_id" : "MkJHHpjFQtKmeYYzPa3n7A",
      "_score" : 2.8561504,
      "_source":{"id":"addr:47.793346;7.231948","house_number":"20","street":{"id":"street:683750960S","street_name":"Rue Poniatowski","name":"Rue Poniatowski, 68310 Wittelsheim","administrative_region":{"id":"admin:68375","level":8,"name":"Wittelsheim","zip_code":"68310","weight":1},"weight":1},"name":"20 Rue Poniatowski, 68310 Wittelsheim","coord":{"lat":47.793346,"lon":7.231948},"weight":1}
    }, {
      "_index" : "munin",
      "_type" : "addr",
      "_id" : "qbpjrhTRS2SDrwzs4ZiSGw",
      "_score" : 2.5294542,
      "_source":{"id":"addr:49.295705;3.585516","house_number":"20","street":{"id":"street:025200095L","street_name":"Rue Poniatowski","name":"Rue Poniatowski, 02220 Mont-Notre-Dame","administrative_region":{"id":"admin:02520","level":8,"name":"Mont-Notre-Dame","zip_code":"02220","weight":1},"weight":1},"name":"20 Rue Poniatowski, 02220 Mont-Notre-Dame","coord":{"lat":49.295705,"lon":3.585516},"weight":1}
    }, {
      "_index" : "munin",
      "_type" : "admin",
      "_id" : "nBcljPQkT9u8pDFYWD9ExQ",
      "_score" : 1.8294557,
      "_source":{"id":"admin:18033","level":8,"name":"Bourges","zip_code":"18000","weight":20539}
    }, {
      "_index" : "munin",
      "_type" : "addr",
      "_id" : "1Zsv5LK_S6aYlJ2nUzia7w",
      "_score" : 1.8279884,
      "_source":{"id":"addr:43.287716;5.451867","house_number":"20","street":{"id":"street:132116836Y","street_name":"Boulevard Pascal","name":"Boulevard Pascal, 13011 Marseille","administrative_region":{"id":"admin:13211","level":8,"name":"Marseille","zip_code":"13011","weight":1},"weight":1},"name":"20 Boulevard Pascal, 13011 Marseille","coord":{"lat":43.287716,"lon":5.451867},"weight":1}
    }, {
      "_index" : "munin",
      "_type" : "addr",
      "_id" : "5D3tsaboS5y91mB9Qzn6ag",
      "_score" : 1.8279884,
      "_source":{"id":"addr:43.296621;-0.364129","house_number":"20","street":{"id":"street:644450380V","street_name":"Boulevard Barbanègre","name":"Boulevard Barbanègre, 64000 Pau","administrative_region":{"id":"admin:64445","level":8,"name":"Pau","zip_code":"64000","weight":1},"weight":1},"name":"20 Boulevard Barbanègre, 64000 Pau","coord":{"lat":43.296621,"lon":-0.364129},"weight":1}
    }, {
      "_index" : "munin",
      "_type" : "addr",
      "_id" : "WHssNyH8RxSDTXWiqBKYNA",
      "_score" : 1.8279884,
      "_source":{"id":"addr:48.840804;2.26583","house_number":"20","street":{"id":"street:751163465H","street_name":"Boulevard Exelmans","name":"Boulevard Exelmans, 75016 Paris","administrative_region":{"id":"admin:75116","level":8,"name":"Paris","zip_code":"75016","weight":1},"weight":1},"name":"20 Boulevard Exelmans, 75016 Paris","coord":{"lat":48.840804,"lon":2.26583},"weight":1}
    }, {
      "_index" : "munin",
      "_type" : "addr",
      "_id" : "y41cmdHCRWW5bwtx-oDyNQ",
      "_score" : 1.8219174,
      "_source":{"id":"addr:43.116026;1.609917","house_number":"20","street":{"id":"street:092250480P","street_name":"Rue Boulbonne","name":"Rue Boulbonne, 09100 Pamiers","administrative_region":{"id":"admin:09225","level":8,"name":"Pamiers","zip_code":"09100","weight":1},"weight":1},"name":"20 Rue Boulbonne, 09100 Pamiers","coord":{"lat":43.116026,"lon":1.609917},"weight":1}
    }, {
      "_index" : "munin",
      "_type" : "addr",
      "_id" : "QGBnH8dFSDa85KCeNTm5rg",
      "_score" : 1.8219174,
      "_source":{"id":"addr:43.328046;3.046292","house_number":"20","street":{"id":"street:340520420V","street_name":"Boulevard Pasteur","name":"Boulevard Pasteur, 34310 Capestang","administrative_region":{"id":"admin:34052","level":8,"name":"Capestang","zip_code":"34310","weight":1},"weight":1},"name":"20 Boulevard Pasteur, 34310 Capestang","coord":{"lat":43.328046,"lon":3.046292},"weight":1}
    } ]
  }
}
]

real	0m0.090s
user	0m0.012s
sys	0m0.004s
```
and
```
{
    "sort" : [
        {
            "_geo_distance" : {
                "coord" : { "lat": 48.82964, "lon": 2.39137 },
                "order" : "asc",
                "unit" : "m"
            }
        }
    ],
    "query": {
        "filtered": {
            "query": {
                "match_all": { }
            },
            "filter": {
                "geohash_cell" : {
                    "coord" : {
                        "lat" : 48.82964,
                        "lon" : 2.39137
                    },
                    "precision" : "1km",
                    "neighbors": true
                }
            }
        }
    }
}

Response {200, content-length: 754, content-type: application/json; charset=UTF-8, {
  "took" : 7,
  "timed_out" : false,
  "_shards" : {
    "total" : 16,
    "successful" : 16,
    "failed" : 0
  },
  "hits" : {
    "total" : 188,
    "max_score" : null,
    "hits" : [ {
      "_index" : "munin",
      "_type" : "addr",
      "_id" : "OG0gGu91Spmn9FVIDw1GgQ",
      "_score" : null,
      "_source":{"id":"addr:48.829647;2.391372","house_number":"20","street":{"id":"street:751127597M","street_name":"Boulevard Poniatowski","name":"Boulevard Poniatowski, 75012 Paris","administrative_region":{"id":"admin:75112","level":8,"name":"Paris","zip_code":"75012","weight":1},"weight":1},"name":"20 Boulevard Poniatowski, 75012 Paris","coord":{"lat":48.829647,"lon":2.391372},"weight":1},
      "sort" : [ 0.7889319987940349 ]
    } ]
  }
}
]

real	0m0.032s
user	0m0.016s
sys	0m0.000s
```

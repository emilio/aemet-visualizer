extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod formats;

#[derive(Debug, Serialize, Deserialize)]
struct SchemaEntry {
    year: String,
    is_aggregate: Option<String>,
    stations: Vec<formats::Station>,
}

fn main() {
    use std::io::Write;

    let directory = std::env::args().skip(1).next().expect("Expected one argument");
    let directory = std::path::Path::new(&directory);
    let data = formats::YearlyData::all_from_manifest_dir(formats::AggregateDataProcessing::Normalize);

    let mut schema = vec![];
    for d in data {
        let mut f = std::fs::File::create(directory.join(format!("{}.json", &d.year)))
            .expect("Couldn't open data file for writing");


        f.write(serde_json::to_string_pretty(&d).unwrap().as_bytes()).unwrap();

        schema.push(SchemaEntry {
            year: d.year,
            is_aggregate: d.is_aggregate,
            stations: d.stations,
        });
    }

    let mut f = std::fs::File::create(directory.join("schema.json"))
        .expect("Couldn't open schema file for writing");
    f.write(serde_json::to_string_pretty(&schema).unwrap().as_bytes()).unwrap();
}

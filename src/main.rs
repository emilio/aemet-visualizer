extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod formats;

fn main() {
    let data = formats::YearlyData::all_from_manifest_dir(false);
    print!("{}", serde_json::to_string_pretty(&data).unwrap());
}

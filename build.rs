extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

#[path = "src/formats.rs"]
mod formats;

fn main() {
    use std::{fs, env, path};
    use std::io::Write;

    let data = formats::YearlyData::all_from_manifest_dir();
    let out_dir = env::var("OUT_DIR").unwrap();
    let path = path::Path::new(&out_dir).join("data.json");
    fs::File::create(&path)
        .unwrap()
        .write_all(serde_json::to_string(&data).unwrap().as_bytes())
        .unwrap();
}

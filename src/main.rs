use meteo_analyzer::formats::Station;

use csv;
use std::io;

fn main() {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b';')
        .from_reader(io::stdin());
    for result in reader.deserialize() {
        let station: Station = result.unwrap();
        println!("{:?}", station);
    }
}

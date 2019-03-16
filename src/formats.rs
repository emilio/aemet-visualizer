//! Different CSV formats as described in:
//!
//! http://www.aemet.es/documentos/es/datos_abiertos/Estadisticas/Estadisticas_meteorofenologicas/evmf_formatos.pdf

use serde::{de, ser};

#[derive(Debug, Deserialize, Serialize)]
pub struct Meters(pub u32);

#[derive(Debug)]
pub enum LongitudeDirection {
    East,
    West,
}

#[derive(Debug)]
pub struct CardinalPoint {
    degrees: u32,
    minutes: u32,
    seconds: u32,
}

impl CardinalPoint {
    fn to_string(&self) -> String {
        format!("{:02}{:02}{:02}", self.degrees, self.minutes, self.seconds,)
    }

    fn from_str(s: &str) -> Result<Self, &'static str> {
        if s.len() != 6 {
            return Err("Invalid length for CardinalPoint");
        }

        fn parse(s: &str) -> Result<u32, &'static str> {
            s.parse()
                .map_err(|_| "Invalid component for cardinal point")
        }

        let degrees = parse(&s[0..2])?;
        let minutes = parse(&s[2..4])?;
        let seconds = parse(&s[4..6])?;

        Ok(Self {
            degrees,
            minutes,
            seconds,
        })
    }
}

#[derive(Debug)]
pub struct Longitude {
    point: CardinalPoint,
    direction: LongitudeDirection,
}

impl ser::Serialize for Longitude {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let direction = match self.direction {
            LongitudeDirection::East => 1,
            LongitudeDirection::West => 2,
        };
        format!("{}{}", self.point.to_string(), direction).serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for Longitude {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s: String = de::Deserialize::deserialize(deserializer)?;
        if s.len() != 7 {
            return Err(de::Error::custom("Invalid length for Longitude"));
        }
        let point = CardinalPoint::from_str(&s[0..6]).map_err(de::Error::custom)?;
        let direction = match s.as_bytes()[6] {
            b'1' => LongitudeDirection::East,
            b'2' => LongitudeDirection::West,
            _ => return Err(de::Error::custom("Invalid longitude direction")),
        };
        Ok(Self { point, direction })
    }
}

#[derive(Debug)]
pub struct Latitude(pub CardinalPoint);

impl ser::Serialize for Latitude {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        self.0.to_string().serialize(serializer)
    }
}

impl<'de> de::Deserialize<'de> for Latitude {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s: String = de::Deserialize::deserialize(deserializer)?;
        let point = CardinalPoint::from_str(&s).map_err(de::Error::custom)?;
        Ok(Latitude(point))
    }
}

/// "Maestro climatológico"
#[derive(Debug, Deserialize, Serialize)]
pub struct Station {
    #[serde(rename = "INDICATIVO")]
    pub id: String,
    #[serde(rename = "NOMBRE")]
    pub name: String,
    #[serde(rename = "PROVINCIA")]
    pub province: String,
    #[serde(rename = "MUNICIPIO")]
    pub city: String,
    #[serde(rename = "ALTITUD")]
    pub altitude: Meters,
    #[serde(rename = "LONGITUD")]
    pub longitude: Longitude,
    #[serde(rename = "LATITUD")]
    pub latitude: Latitude,
    #[serde(rename = "DATUM")]
    pub datum: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_csv_file<Record>(path_template: &str)
    where
        Record: for<'de> serde::de::Deserialize<'de>,
        Record: std::fmt::Debug,
    {
        use csv;
        use std::fs;

        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/data");
        for year in &["2016", "2017", "2018"] {
            let path = format!("{}/{}/{}", path, year, path_template.replace("{}", year));
            let file = match fs::File::open(&path) {
                Ok(file) => file,
                Err(e) => panic!("Could not open {}: {:?}", path, e),
            };
            let reader = std::io::BufReader::new(file);
            let mut reader =
                csv::ReaderBuilder::new().delimiter(b';').from_reader(reader);
            for record in reader.deserialize() {
                let record: Record = record.unwrap();
                println!("{:?}", record);
            }
        }
    }

    #[test]
    fn test_station() {
        read_csv_file::<Station>("Maestro_Climatologico_{}.csv");
    }
}

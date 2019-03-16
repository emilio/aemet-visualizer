//! Different CSV formats as described in:
//!
//! http://www.aemet.es/documentos/es/datos_abiertos/Estadisticas/Estadisticas_meteorofenologicas/evmf_formatos.pdf

use serde::{de, ser};

#[derive(Debug, Deserialize, Serialize)]
pub struct Meters(pub f32);

#[derive(Debug, Deserialize, Serialize)]
pub struct Celsius(pub f32);

#[derive(Debug, Deserialize, Serialize)]
pub struct Mm(pub f32);

#[derive(Debug, Deserialize, Serialize)]
pub struct TenthsOfMm(pub f32);

#[derive(Debug, Deserialize, Serialize)]
pub struct Percentage(pub f32);

#[derive(Debug, Deserialize, Serialize)]
pub struct TenthsOfHectoPascal(pub f32);

#[derive(Debug, Deserialize, Serialize)]
pub struct Days(pub u32);

#[derive(Debug, Deserialize, Serialize)]
pub struct Hours(pub f32);

#[derive(Debug, Deserialize, Serialize)]
pub struct Kilometers(pub f32);

#[derive(Debug, Deserialize, Serialize)]
pub struct KilometersPerHour(pub f32);

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

/// "Formato F1", with the unit of the statistical data.
#[derive(Debug, Deserialize, Serialize)]
pub struct F1<Data> {
    #[serde(rename = "Indicativo")]
    pub station_id: String,
    #[serde(rename = "enero")]
    pub january: Option<Data>,
    #[serde(rename = "febrero")]
    pub february: Option<Data>,
    #[serde(rename = "marzo")]
    pub march: Option<Data>,
    #[serde(rename = "abril")]
    pub april: Option<Data>,
    #[serde(rename = "mayo")]
    pub may: Option<Data>,
    #[serde(rename = "junio")]
    pub june: Option<Data>,
    #[serde(rename = "julio")]
    pub july: Option<Data>,
    #[serde(rename = "agosto")]
    pub august: Option<Data>,
    #[serde(rename = "septiembre")]
    pub september: Option<Data>,
    #[serde(rename = "octubre")]
    pub october: Option<Data>,
    #[serde(rename = "noviembre")]
    pub november: Option<Data>,
    #[serde(rename = "diciembre")]
    pub december: Option<Data>,
    #[serde(rename = "anual")]
    pub yearly: Option<Data>,
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

    // http://www.aemet.es/documentos/es/datos_abiertos/Estadisticas/Estadisticas_meteorofenologicas/evmf_parametros.pdf
    #[test]
    fn monthly_formats() {
        // Temperature
        read_csv_file::<F1<Celsius>>("mensuales/TM_MES_{}.csv");
        read_csv_file::<F1<Celsius>>("mensuales/TM_MAX_{}.csv");
        read_csv_file::<F1<Celsius>>("mensuales/TM_MIN_{}.csv");
        // read_csv_file::<F2<Celsius>>("mensuales/TA_MAX_{}.csv");
        // read_csv_file::<F2<Celsius>>("mensuales/TA_MIN_{}.csv");
        read_csv_file::<F1<Celsius>>("mensuales/TS_MIN_{}.csv");
        read_csv_file::<F1<Celsius>>("mensuales/TI_MAX_{}.csv");
        read_csv_file::<F1<Days>>("mensuales/NT_30_{}.csv");
        read_csv_file::<F1<Days>>("mensuales/NT_00_{}.csv");

        // Rain
        read_csv_file::<F1<Mm>>("mensuales/P_MES_{}.csv");
        // read_csv_file::<F2<Mm>>("mensuales/P_MAX_{}.csv");
        read_csv_file::<F1<Days>>("mensuales/NP_001_{}.csv");
        read_csv_file::<F1<Days>>("mensuales/NP_010_{}.csv");
        read_csv_file::<F1<Days>>("mensuales/NP_100_{}.csv");
        read_csv_file::<F1<Days>>("mensuales/NP_300_{}.csv");

        // Humidity
        read_csv_file::<F1<Percentage>>("mensuales/HR_{}.csv");
        read_csv_file::<F1<TenthsOfHectoPascal>>("mensuales/E_{}.csv");

        // Days of rain/snow/storm/fog/...
        read_csv_file::<F1<Days>>("mensuales/N_LLU_{}.csv");
        read_csv_file::<F1<Days>>("mensuales/N_NIE_{}.csv");
        read_csv_file::<F1<Days>>("mensuales/N_GRA_{}.csv");
        read_csv_file::<F1<Days>>("mensuales/N_TOR_{}.csv");
        read_csv_file::<F1<Days>>("mensuales/N_FOG_{}.csv");
        read_csv_file::<F1<Days>>("mensuales/N_DES_{}.csv");
        read_csv_file::<F1<Days>>("mensuales/N_NUB_{}.csv");
        read_csv_file::<F1<Days>>("mensuales/N_CUB_{}.csv");

        // Hours of sun.
        read_csv_file::<F1<Hours>>("mensuales/INSO_{}.csv");
        read_csv_file::<F1<Percentage>>("mensuales/P_SOL_{}.csv");
        // Global radiation
        // TODO: What's this unit even? decenas the Kj.m^{-2}
        // read_csv_file::<F1<XXX>>("mensuales/GLO_{}.csv");

        // Evaporation
        read_csv_file::<F1<TenthsOfMm>>("mensuales/EVAP_{}.csv");

        read_csv_file::<F1<Kilometers>>("mensuales/W_REC_{}.csv");
        // read_csv_file::<F3>("mensuales/W_RACHA_{}.csv");

        // Wind speed greater than.
        read_csv_file::<F1<Days>>("mensuales/NW_55_{}.csv");
        read_csv_file::<F1<Days>>("mensuales/NW_91_{}.csv");

        // Average wind speed.
        read_csv_file::<F1<KilometersPerHour>>("mensuales/W_MED_{}.csv");

        // Pressure.
        read_csv_file::<F1<TenthsOfHectoPascal>>("mensuales/Q_MED_{}.csv");
        // read_csv_file::<F2<TenthsOfHectoPascal>>("mensuales/Q_MAX_{}.csv");
        // read_csv_file::<F2<TenthsOfHectoPascal>>("mensuales/Q_MIN_{}.csv");
        read_csv_file::<F1<TenthsOfHectoPascal>>("mensuales/Q_MAR_{}.csv");

        // Temperature under sea level.
        read_csv_file::<F1<Celsius>>("mensuales/TS_10_{}.csv");
        read_csv_file::<F1<Celsius>>("mensuales/TS_20_{}.csv");
        read_csv_file::<F1<Celsius>>("mensuales/TS_50_{}.csv");

        // Visibility.
        read_csv_file::<F1<Days>>("mensuales/NV_0050_{}.csv");
        read_csv_file::<F1<Days>>("mensuales/NV_0100_{}.csv");
        read_csv_file::<F1<Days>>("mensuales/NV_1000_{}.csv");
    }
}

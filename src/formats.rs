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

// FIXME: This should be u32, but the aggregate data contains floats with a
// bunch of .0's
#[derive(Debug, Deserialize, Serialize)]
pub struct Days(pub f32);

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
    #[serde(alias = "INDICATIVO")]
    pub id: String,
    #[serde(alias = "NOMBRE")]
    pub name: String,
    #[serde(alias = "PROVINCIA")]
    pub province: String,
    #[serde(alias = "MUNICIPIO")]
    pub city: String,
    #[serde(alias = "ALTITUD")]
    pub altitude: Meters,
    #[serde(alias = "LONGITUD")]
    pub longitude: Longitude,
    #[serde(alias = "LATITUD")]
    pub latitude: Latitude,
    #[serde(alias = "DATUM")]
    pub datum: String,
}

/// "Formato F1", with the unit of the statistical data.
#[derive(Debug, Deserialize, Serialize)]
pub struct F1<Data> {
    #[serde(alias = "Indicativo")]
    pub station_id: String,
    #[serde(alias = "enero")]
    pub january: Option<Data>,
    #[serde(alias = "febrero")]
    pub february: Option<Data>,
    #[serde(alias = "marzo")]
    pub march: Option<Data>,
    #[serde(alias = "abril")]
    pub april: Option<Data>,
    #[serde(alias = "mayo")]
    pub may: Option<Data>,
    #[serde(alias = "junio")]
    pub june: Option<Data>,
    #[serde(alias = "julio")]
    pub july: Option<Data>,
    #[serde(alias = "agosto")]
    pub august: Option<Data>,
    #[serde(alias = "septiembre")]
    pub september: Option<Data>,
    #[serde(alias = "octubre")]
    pub october: Option<Data>,
    #[serde(alias = "noviembre")]
    pub november: Option<Data>,
    #[serde(alias = "diciembre")]
    pub december: Option<Data>,
    #[serde(alias = "anual")]
    pub yearly: Option<Data>,
}

/// The yearly data for all the meteorological stations.
///
/// http://www.aemet.es/documentos/es/datos_abiertos/Estadisticas/Estadisticas_meteorofenologicas/evmf_parametros.pdf
#[derive(Debug, Deserialize, Serialize)]
pub struct YearlyData {
    pub year: u32,

    pub stations: Vec<Station>,

    pub average_temperature: Vec<F1<Celsius>>,
    pub average_max_temperature: Vec<F1<Celsius>>,
    pub average_min_temperature: Vec<F1<Celsius>>,

    // pub absolute_max_temperature: F2<Celsius>,
    // pub absolute_min_temperature: F2<Celsius>,

    pub higher_min_temperature: Vec<F1<Celsius>>,
    pub lower_max_temperature: Vec<F1<Celsius>>,

    pub number_of_days_gteq_30_celsius: Vec<F1<Days>>,
    pub number_of_days_lteq_0_celsius: Vec<F1<Days>>,

    pub total_rain: Vec<F1<Mm>>,
    // pub max_rain: Vec<F2<Mm>>,

    pub days_with_appreciable_rain: Vec<F1<Days>>,
    pub days_with_rain_gteq_1_mm: Vec<F1<Days>>,
    pub days_with_rain_gteq_10_mm: Vec<F1<Days>>,
    pub days_with_rain_gteq_30_mm: Vec<F1<Days>>,

    pub average_relative_humidity: Vec<F1<Percentage>>,
    pub average_vapor_tension: Vec<F1<TenthsOfHectoPascal>>,

    pub days_of_rain: Vec<F1<Days>>,
    pub days_of_snow: Vec<F1<Days>>,
    pub days_of_hail: Vec<F1<Days>>,
    pub days_of_storm: Vec<F1<Days>>,
    pub days_of_fog: Vec<F1<Days>>,
    pub clear_days: Vec<F1<Days>>,
    pub cloudy_days: Vec<F1<Days>>,
    pub covered_days: Vec<F1<Days>>,

    pub hours_of_sun: Vec<F1<Hours>>,
    pub average_percentage_against_theoric_insolation: Vec<F1<Percentage>>,

    // TODO: What's this unit even? Tenths of Kj.m^{-2}
    // pub global_radiation: Vec<F1<XXX>>,

    pub evaporation: Vec<F1<TenthsOfMm>>,

    pub average_distance: Vec<F1<Kilometers>>,

    // pub biggest_gust_of_wind: Vec<F3>,

    pub days_with_wind_greater_than_55_km_per_hour: Vec<F1<Days>>,
    pub days_with_wind_greater_than_91_km_per_hour: Vec<F1<Days>>,

    pub average_wind_speed: Vec<F1<KilometersPerHour>>,

    pub average_pressure: Vec<F1<TenthsOfHectoPascal>>,
    // pub max_pressure: Vec<F2<TenthsOfHectoPascal>>,
    // pub min_pressure: Vec<F2<TenthsOfHectoPascal>>,
    pub average_pressure_sea_level: Vec<F1<TenthsOfHectoPascal>>,

    pub average_temperature_under_10_cm: Vec<F1<Celsius>>,
    pub average_temperature_under_20_cm: Vec<F1<Celsius>>,
    pub average_temperature_under_50_cm: Vec<F1<Celsius>>,

    pub days_with_visibility_lt_50_m: Vec<F1<Days>>,
    pub days_with_visibility_gteq_50_m_lt_100_m: Vec<F1<Days>>,
    pub days_with_visibility_gteq_100_m_lt_1000_m: Vec<F1<Days>>,
}


impl YearlyData {
    /// Reads the yearly data from a given csv directory.
    ///
    /// This panics on error, assuming that data is under control.
    pub fn from_csv(directory: &std::path::Path, year: u32) -> Self {
        use std::{fs, io};

        fn read_csv_file<Record>(path: &std::path::Path) -> Vec<Record>
        where
            Record: for<'de> serde::de::Deserialize<'de>,
        {
            let file = match fs::File::open(&path) {
                Ok(file) => file,
                Err(e) => panic!("Could not open {}: {:?}", path.display(), e),
            };
            let reader = io::BufReader::new(file);
            let mut reader =
                csv::ReaderBuilder::new().delimiter(b';').from_reader(reader);
            reader.deserialize().map(|record| record.unwrap()).collect()
        }

        macro_rules! read_monthly {
            ($name: expr) => {{
                let path = directory
                    .join("mensuales")
                    .join(format!("{}_{}.csv", $name, year));
                read_csv_file(&path)
            }}
        };

        Self {
            year,
            stations: read_csv_file(&directory.join(format!("Maestro_Climatologico_{}.csv", year))),
            average_temperature: read_monthly!("TM_MES"),
            average_max_temperature: read_monthly!("TM_MAX"),
            average_min_temperature: read_monthly!("TM_MIN"),
            higher_min_temperature: read_monthly!("TS_MIN"),
            lower_max_temperature: read_monthly!("TI_MAX"),
            number_of_days_gteq_30_celsius: read_monthly!("NT_30"),
            number_of_days_lteq_0_celsius: read_monthly!("NT_00"),
            total_rain: read_monthly!("P_MES"),
            days_with_appreciable_rain: read_monthly!("NP_001"),
            days_with_rain_gteq_1_mm: read_monthly!("NP_010"),
            days_with_rain_gteq_10_mm: read_monthly!("NP_100"),
            days_with_rain_gteq_30_mm: read_monthly!("NP_300"),

            average_relative_humidity: read_monthly!("HR"),
            average_vapor_tension: read_monthly!("E"),

            days_of_rain: read_monthly!("N_LLU"),
            days_of_snow: read_monthly!("N_NIE"),
            days_of_hail: read_monthly!("N_GRA"),
            days_of_storm: read_monthly!("N_TOR"),
            days_of_fog: read_monthly!("N_FOG"),
            clear_days: read_monthly!("N_DES"),
            cloudy_days: read_monthly!("N_NUB"),
            covered_days: read_monthly!("N_CUB"),

            hours_of_sun: read_monthly!("INSO"),
            average_percentage_against_theoric_insolation: read_monthly!("P_SOL"),

            evaporation: read_monthly!("EVAP"),
            average_distance: read_monthly!("W_REC"),

            days_with_wind_greater_than_55_km_per_hour: read_monthly!("NW_55"),
            days_with_wind_greater_than_91_km_per_hour: read_monthly!("NW_91"),

            average_wind_speed: read_monthly!("W_MED"),

            average_pressure: read_monthly!("Q_MED"),
            average_pressure_sea_level: read_monthly!("Q_MAR"),

            average_temperature_under_10_cm: read_monthly!("TS_10"),
            average_temperature_under_20_cm: read_monthly!("TS_20"),
            average_temperature_under_50_cm: read_monthly!("TS_50"),

            days_with_visibility_lt_50_m: read_monthly!("NV_0050"),
            days_with_visibility_gteq_50_m_lt_100_m: read_monthly!("NV_0100"),
            days_with_visibility_gteq_100_m_lt_1000_m: read_monthly!("NV_1000"),
        }
    }

    /// Gets all the data from the in-repo data.
    pub fn all_from_manifest_dir() -> Vec<Self> {
        use std::path::Path;

        macro_rules! yearly_data {
            ($year:tt) => {{
                YearlyData::from_csv(
                    Path::new(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/data/",
                        stringify!($year)
                    )),
                    $year,
                )
            }}
        }

        vec![
            yearly_data!(2016),
            yearly_data!(2017),
            yearly_data!(2018),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        YearlyData::all_from_manifest_dir();
    }
}

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
#[serde(bound = "Option<Data>: serde::Serialize + for<'a> serde::Deserialize<'a>")]
pub struct F1<Data> {
    #[serde(alias = "Indicativo")]
    pub station_id: String,
    #[serde(flatten)]
    pub yearly: PerYear<Data>,
}

/// "Formato F4", for aggregates.
#[derive(Debug, Deserialize, Serialize)]
pub enum AggregateParameter {
    /// The number of samples in this aggregate.
    #[serde(alias = "N")]
    SampleCount,
    Min,
    Q1,
    Q2,
    Q3,
    Q4,
    Max,
    #[serde(alias = "Mn")]
    Median,
    #[serde(alias = "Md")]
    Average,
    #[serde(alias = "S")]
    StdDev,
    Cv,
}

/// "Formato F4", for aggregates.
#[derive(Debug, Deserialize, Serialize)]
#[serde(bound = "Option<Data>: serde::Serialize + for<'a> serde::Deserialize<'a>")]
pub struct F4<Data> {
    #[serde(alias = "Indicativo")]
    pub station_id: String,
    #[serde(alias = "parámetro")]
    pub parameter: AggregateParameter,
    #[serde(flatten)]
    pub yearly: PerYear<Data>,
}

impl<Data> F4<Data> {
    /// Turns an aggregate into the yearly data and the aggregate parameter it
    /// represents.
    pub fn into_f1(self) -> (F1<Data>, AggregateParameter) {
        (
            F1 {
                station_id: self.station_id,
                yearly: self.yearly,
            },
            self.parameter,
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(bound = "Option<Data>: serde::Serialize + for<'a> serde::Deserialize<'a>")]
pub struct PerYear<Data> {
    #[serde(deserialize_with = "csv::invalid_option", alias = "enero")]
    pub january: Option<Data>,
    #[serde(deserialize_with = "csv::invalid_option", alias = "febrero")]
    pub february: Option<Data>,
    #[serde(deserialize_with = "csv::invalid_option", alias = "marzo")]
    pub march: Option<Data>,
    #[serde(deserialize_with = "csv::invalid_option", alias = "abril")]
    pub april: Option<Data>,
    #[serde(deserialize_with = "csv::invalid_option", alias = "mayo")]
    pub may: Option<Data>,
    #[serde(deserialize_with = "csv::invalid_option", alias = "junio")]
    pub june: Option<Data>,
    #[serde(deserialize_with = "csv::invalid_option", alias = "julio")]
    pub july: Option<Data>,
    #[serde(deserialize_with = "csv::invalid_option", alias = "agosto")]
    pub august: Option<Data>,
    #[serde(deserialize_with = "csv::invalid_option", alias = "septiembre")]
    pub september: Option<Data>,
    #[serde(deserialize_with = "csv::invalid_option", alias = "octubre")]
    pub october: Option<Data>,
    #[serde(deserialize_with = "csv::invalid_option", alias = "noviembre")]
    pub november: Option<Data>,
    #[serde(deserialize_with = "csv::invalid_option", alias = "diciembre")]
    pub december: Option<Data>,
    #[serde(deserialize_with = "csv::invalid_option", alias = "anual")]
    pub yearly: Option<Data>,
}

macro_rules! enumerate_record_kinds {
    ($m:ident) => {
        $m! {
            [average_temperature, Celsius, "TM_MES"],
            [average_max_temperature, Celsius, "TM_MAX"],
            [average_min_temperature, Celsius, "TM_MIN"],

            // [absolute_max_temperature, WithDate<Celsius>, "TA_MAX"],
            // [absolute_min_temperature, WithDate<Celsius>, "TA_MIN"],

            [higher_min_temperature, Celsius, "TS_MIN"],
            [lower_max_temperature, Celsius, "TI_MAX"],

            [number_of_days_gteq_30_celsius, Days, "NT_30"],
            [number_of_days_lteq_0_celsius, Days, "NT_00"],

            [total_rain, Mm, "P_MES"],
            // [max_rain, WithDate<Mm>, "P_MAX"],

            [days_with_appreciable_rain, Days, "NP_001"],
            [days_with_rain_gteq_1_mm, Days, "NP_010"],
            [days_with_rain_gteq_10_mm, Days, "NP_100"],
            [days_with_rain_gteq_30_mm, Days, "NP_300"],

            [average_relative_humidity, Percentage, "HR"],
            [average_vapor_tension, TenthsOfHectoPascal, "E"],

            [days_of_rain, Days, "N_LLU"],
            [days_of_snow, Days, "N_NIE"],
            [days_of_hail, Days, "N_GRA"],
            [days_of_storm, Days, "N_TOR"],
            [days_of_fog, Days, "N_FOG"],
            [clear_days, Days, "N_DES"],
            [cloudy_days, Days, "N_NUB"],
            [covered_days, Days, "N_CUB"],

            [hours_of_sun, Hours, "INSO"],
            [average_percentage_against_theoric_insolation, Percentage, "P_SOL"],

            // TODO, What's this unit even? Tenths of Kj.m^{-2}
            // [global_radiation, XXX, "GLO"],

            [evaporation, TenthsOfMm, "EVAP"],

            [average_distance, Kilometers, "W_REC"],

            // [biggest_gust_of_wind, F3, "W_RACHA"],

            [days_with_wind_greater_than_55_km_per_hour, Days, "NW_55"],
            [days_with_wind_greater_than_91_km_per_hour, Days, "NW_91"],

            [average_wind_speed, KilometersPerHour, "W_MED"],

            [average_pressure, TenthsOfHectoPascal, "Q_MED"],
            // [max_pressure, WithDate<TenthsOfHectoPascal>, "Q_MAX"],
            // [min_pressure, WithDate<TenthsOfHectoPascal>, "Q_MIN"],
            [average_pressure_sea_level, TenthsOfHectoPascal, "Q_MAR"],

            [average_temperature_under_10_cm, Celsius, "TS_10"],
            [average_temperature_under_20_cm, Celsius, "TS_20"],
            [average_temperature_under_50_cm, Celsius, "TS_50"],

            [days_with_visibility_lt_50_m, Days, "NV_0050"],
            [days_with_visibility_gteq_50_m_lt_100_m, Days, "NV_0100"],
            [days_with_visibility_gteq_100_m_lt_1000_m, Days, "NV_1000"],
        }
    }
}

macro_rules! declare_aggregate_data {
    ($([$name:ident, $ty:ty, $f:expr],)*) => {
        /// The yearly data for all the meteorological stations.
        ///
        /// http://www.aemet.es/documentos/es/datos_abiertos/Estadisticas/Estadisticas_meteorofenologicas/evmf_parametros.pdf
        #[derive(Debug, Default, Deserialize, Serialize)]
        pub struct AggregateData {
            $(
                pub $name: Vec<F4<$ty>>,
            )*
        }
    }
}

enumerate_record_kinds!(declare_aggregate_data);

macro_rules! declare_yearly_data {
    ($([$name:ident, $ty:ty, $f:expr],)*) => {
        /// The yearly data for all the meteorological stations.
        ///
        /// http://www.aemet.es/documentos/es/datos_abiertos/Estadisticas/Estadisticas_meteorofenologicas/evmf_parametros.pdf
        #[derive(Debug, Deserialize, Serialize)]
        pub struct YearlyData {
            pub year: u32,
            pub stations: Vec<Station>,
            $(
                pub $name: Vec<F1<$ty>>,
            )*
            pub aggregate: AggregateData,
        }
    }
}

enumerate_record_kinds!(declare_yearly_data);

impl YearlyData {
    /// Reads the yearly data from a given csv directory.
    ///
    /// This panics on error, assuming that data is under control.
    pub fn from_csv(
        directory: &std::path::Path,
        year: u32,
        with_aggregate: bool,
    ) -> Self {
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
            reader.deserialize().map(|record| {
                match record {
                    Ok(record) => record,
                    Err(e) => panic!("Errored while parsing {}: {:?}", path.display(), e),
                }
            }).collect()
        }

        macro_rules! read {
            ($([$name:ident, $ty:ty, $f:expr],)*) => {
                Self {
                    year,
                    stations: read_csv_file(&directory.join(format!("Maestro_Climatologico_{}.csv", year))),
                    $(
                        $name: {
                            let path = directory
                                .join("mensuales")
                                .join(format!("{}_{}.csv", $f, year));
                            read_csv_file(&path)
                        },
                    )*

                    aggregate: if with_aggregate {
                        AggregateData {
                            $(
                                $name: {
                                    let path = directory
                                        .join("normales")
                                        .join(format!("{}_1981_2010.csv", $f));
                                    read_csv_file(&path)
                                },
                            )*
                        }
                    } else {
                        AggregateData::default()
                    },
                }
            }
        };

        enumerate_record_kinds!(read)
    }

    /// Gets all the data from the in-repo data.
    pub fn all_from_manifest_dir(with_aggregate: bool) -> Vec<Self> {
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
                    with_aggregate,
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
        YearlyData::all_from_manifest_dir(true);
    }
}

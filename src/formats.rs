//! Different CSV formats as described in:
//!
//! http://www.aemet.es/documentos/es/datos_abiertos/Estadisticas/Estadisticas_meteorofenologicas/evmf_formatos.pdf

use serde::{de, ser};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Meters(pub f32);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Celsius(pub f32);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Mm(pub f32);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TenthsOfMm(pub f32);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Percentage(pub f32);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TenthsOfHectoPascal(pub f32);

// FIXME: This should be u32, but the aggregate data contains floats with a
// bunch of .0's
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Days(pub f32);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Hours(pub f32);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Kilometers(pub f32);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KilometersPerHour(pub f32);

#[derive(Debug, Clone)]
pub enum LongitudeDirection {
    East,
    West,
}

#[derive(Debug, Clone)]
pub struct CardinalPoint {
    degrees: u32,
    minutes: u32,
    seconds: u32,
}

#[derive(Debug, Serialize, Clone)]
pub struct WithDate<Data> {
    value: Data,
    date: String,
}

macro_rules! forward_with_date_de {
    ($ty: ident) => {
        impl<'de> de::Deserialize<'de> for WithDate<$ty> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                let float = WithDate::<f32>::deserialize(deserializer)?;
                Ok(WithDate {
                    value: $ty(float.value),
                    date: float.date,
                })
            }
        }
    }
}

forward_with_date_de!(Celsius);
forward_with_date_de!(Mm);
forward_with_date_de!(TenthsOfHectoPascal);

impl<'de> de::Deserialize<'de> for WithDate<f32> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s: String = de::Deserialize::deserialize(deserializer)?;
        let mut split = s.split('(');

        let value = match split.next() {
            Some(s) => s.parse().map_err(|_| de::Error::custom("Invalid value in WithDate"))?,
            None => return Err(de::Error::custom("Empty value")),
        };

        let rest = match split.next() {
            Some(s) => s,
            None => return Err(de::Error::custom("No date?")),
        };

        let mut split = rest.split(')');
        let date = match split.next() {
            Some(d) => d,
            None => return Err(de::Error::custom("Incomplete input")),
        };

        match split.next() {
            None => Err(de::Error::custom("Unclosed parenthesis?")),
            Some(s) if !s.is_empty() => Err(de::Error::custom("Extraneous content?")),
            _ => Ok(WithDate {
                value,
                date: date.to_string(),
            }),
        }
    }
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
#[derive(Debug, Deserialize, Serialize, Clone)]
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
#[derive(Debug, Copy, Clone, Deserialize, Serialize, PartialEq, Eq)]
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

/// The parameters that can end up being normalized. Sample count, deviation and
/// coefficient of variation cannot really be normalized, since they're not
/// really in the units we want.
const NORMALIZED_PARAMETERS: [AggregateParameter; 8] = [
    AggregateParameter::Average,
    AggregateParameter::Median,
    AggregateParameter::Min,
    AggregateParameter::Q1,
    AggregateParameter::Q2,
    AggregateParameter::Q3,
    AggregateParameter::Q4,
    AggregateParameter::Max,
];

impl AggregateParameter {
    fn as_human_str(&self) -> &'static str {
        match *self {
            AggregateParameter::SampleCount => "sample count",
            AggregateParameter::Min => "minimum",
            AggregateParameter::Max => "maximum",
            AggregateParameter::Q1 => "quintile 1",
            AggregateParameter::Q2 => "quintile 2",
            AggregateParameter::Q3 => "quintile 3",
            AggregateParameter::Q4 => "quintile 4",
            AggregateParameter::Average => "average",
            AggregateParameter::Median => "median",
            AggregateParameter::StdDev => "standard deviation",
            AggregateParameter::Cv => "coefficient of variation",
        }
    }
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

            [absolute_max_temperature, WithDate<Celsius>, "TA_MAX"],
            [absolute_min_temperature, WithDate<Celsius>, "TA_MIN"],

            [higher_min_temperature, Celsius, "TS_MIN"],
            [lower_max_temperature, Celsius, "TI_MAX"],

            [number_of_days_gteq_30_celsius, Days, "NT_30"],
            [number_of_days_lteq_0_celsius, Days, "NT_00"],

            [total_rain, Mm, "P_MES"],
            [max_rain, WithDate<Mm>, "P_MAX"],

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
            [max_pressure, WithDate<TenthsOfHectoPascal>, "Q_MAX"],
            [min_pressure, WithDate<TenthsOfHectoPascal>, "Q_MIN"],
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
            pub from_year: u32,
            pub to_year: u32,

            $(
                pub $name: Vec<F4<$ty>>,
            )*
        }

        impl AggregateData {
            /// Given an aggregate parameter, returns a new `YearlyData` with
            /// the data that represents that parameter.
            fn normalize_taking(
                &mut self,
                param: AggregateParameter,
                year: String,
                label: String,
            ) -> YearlyData {
                YearlyData {
                    year: label,
                    is_aggregate: Some(year),
                    stations: vec![],

                    $(
                        $name: {
                            // FIXME: Is there a more rusty way to do this?
                            let mut normalized = vec![];
                            for i in (0..self.$name.len()).rev() {
                                if self.$name[i].parameter != param {
                                    continue;
                                }
                                normalized.push(self.$name.remove(i).into_f1().0);
                            }
                            normalized
                        },
                    )*

                    aggregate: AggregateData::default(),
                }
            }
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
            /// A label that describe the year or the year range.
            pub year: String,
            /// Whether the data is a normalized aggregate, and if so from which
            /// dataset.
            pub is_aggregate: Option<String>,
            pub stations: Vec<Station>,
            $(
                pub $name: Vec<F1<$ty>>,
            )*
            pub aggregate: AggregateData,
        }
    }
}

enumerate_record_kinds!(declare_yearly_data);

#[derive(Debug, PartialEq, Eq)]
pub enum AggregateDataProcessing {
    /// Processes no aggregate data.
    No,
    /// Returns the full aggregate data.
    #[allow(dead_code)]
    Full,
    /// Normalizes the aggregate data so that only average / median parameters
    /// show up.
    Normalize,
}

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
                    year: year.to_string(),
                    is_aggregate: None,
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
                            from_year: 1981,
                            to_year: 2010,
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
    pub fn all_from_manifest_dir(
        aggregate_data: AggregateDataProcessing,
    ) -> Vec<Self> {
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
                    aggregate_data != AggregateDataProcessing::No,
                )
            }}
        }

        let mut data = vec![
            yearly_data!(2016),
            yearly_data!(2017),
            yearly_data!(2018),
        ];

        if let AggregateDataProcessing::Normalize = aggregate_data {
            let mut extra = Vec::with_capacity(data.len() * NORMALIZED_PARAMETERS.len());
            for d in &mut data {
                let mut aggregate = std::mem::replace(&mut d.aggregate, Default::default());
                for param in &NORMALIZED_PARAMETERS {
                    extra.push(aggregate.normalize_taking(
                        *param,
                        d.year.clone(),
                        format!(
                            "{} - {} {} ({} dataset)",
                            aggregate.from_year,
                            aggregate.to_year,
                            param.as_human_str(),
                            d.year,
                        ),
                    ));
                }
            }
            data.extend(extra.into_iter());
        }

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        YearlyData::all_from_manifest_dir(AggregateDataProcessing::Full);
    }
}

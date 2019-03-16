(function() {

const kMonths = [
  "january",
  "february",
  "march",
  "april",
  "may",
  "june",
  "july",
  "august",
  "september",
  "october",
  "november",
  "december",
];

const kKnownMetrics = {
  average_temperature: {
    pretty: "Average temperature",
    unit: "Celsius",
  },
  average_max_temperature: {
    pretty: "Average max temperature",
    unit: "Celsius",
  },
  average_min_temperature: {
    pretty: "Average min temperature",
    unit: "Celsius",
  },
  higher_min_temperature: {
    pretty: "Higher min temperature",
    unit: "Celsius",
  },
  lower_max_temperature: {
    pretty: "Lower max temperature",
    unit: "Celsius",
  },
  number_of_days_gteq_30_celsius: {
    pretty: "Number of days with >= 30 degrees celsius",
    unit: "Days",
  },
  number_of_days_lteq_0_celsius: {
    pretty: "Number of days with <= 0 degrees celsius",
    unit: "Days",
  },
  total_rain: {
    pretty: "Total rain",
    unit: "Mm",
  },
  days_with_appreciable_rain: {
    pretty: "Days with appreciable rain (>= 0.1mm)",
    unit: "Days",
    multiplier: 0.1,
  },
  days_with_rain_gteq_1_mm: {
    pretty: "Days with >= 1mm of rain",
    unit: "Days",
  },
  days_with_rain_gteq_10_mm: {
    pretty: "Days with >= 10mm of rain",
    unit: "Days",
  },
  days_with_rain_gteq_30_mm: {
    pretty: "Days with >= 30mm of rain",
    unit: "Days",
  },
  average_relative_humidity: {
    pretty: "Average relative humidity (%)",
    unit: "%",
  },
  average_vapor_tension: {
    pretty: "Average vapor tension",
    unit: "hPa",
    multiplier: 0.1,
  },
  days_of_rain: {
    pretty: "Days of rain",
    unit: "Days",
  },
  days_of_snow: {
    pretty: "Days of snow",
    unit: "Days",
  },
  days_of_hail: {
    pretty: "Days of hail",
    unit: "Days",
  },
  days_of_storm: {
    pretty: "Days of storm",
    unit: "Days",
  },
  days_of_fog: {
    pretty: "Days of fog",
    unit: "Days",
  },
  clear_days: {
    pretty: "Clear days",
    unit: "Days",
  },
  cloudy_days: {
    pretty: "Cloudy days",
    unit: "Days",
  },
  covered_days: {
    pretty: "Covered days",
    unit: "Days",
  },
  hours_of_sun: {
    pretty: "Hours of sun",
    unit: "Hours",
  },
  average_percentage_against_theoric_insolation: {
    pretty: "Average percentage against theoric insolation",
    unit: "%",
  },
  evaporation: {
    pretty: "Evaporation",
    unit: "Mm",
    multiplier: 0.1,
  },
  average_distance: {
    pretty: "Average distance",
    unit: "Km",
  },
  days_with_wind_greater_than_55_km_per_hour: {
    pretty: "Days with wind > 55km/h",
    unit: "Days",
  },
  days_with_wind_greater_than_91_km_per_hour: {
    pretty: "Days with wind > 91km/h",
    unit: "Days",
  },
  average_wind_speed: {
    pretty: "Average wind speed",
    unit: "km/h",
  },
  average_pressure: {
    pretty: "Average pressure",
    unit: "hPa",
    multiplier: 0.1,
  },
  average_pressure_sea_level: {
    pretty: "Average pressure at sea level",
    unit: "hPa",
    multiplier: 0.1,
  },
  average_temperature_under_10_cm: {
    pretty: "Average temperature under 10 cm",
    unit: "Celsius",
    multiplier: 0.1,
  },
  average_temperature_under_20_cm: {
    pretty: "Average temperature under 20 cm",
    unit: "Celsius",
    multiplier: 0.1,
  },
  average_temperature_under_50_cm: {
    pretty: "Average temperature under 50 cm",
    unit: "Celsius",
    multiplier: 0.1,
  },
  days_with_visibility_lt_50_m: {
    pretty: "Days with visibility < 50m",
    unit: "Days",
  },
  days_with_visibility_gteq_50_m_lt_100_m: {
    pretty: "Days with visibility < 50m",
    unit: "Days",
  },
  days_with_visibility_gteq_100_m_lt_1000_m: {
    pretty: "Days with visibility >= 100m < 1000m",
    unit: "Days",
  },
};

const kKnownUnits = (function() {
  const units = {};
  for (const metric in kKnownMetrics) {
    const m = kKnownMetrics[metric];
    if (!units[m.unit])
      units[m.unit] = [];
    units[m.unit].push(metric);
  }
  return units;
})();

window.Charts = class Charts {
  constructor(chartContainer, controls, data) {
    this.chartContainer = chartContainer;
    this.controls = controls;
    this.data = data;
    this.stations = {};
    this.charts = {};
    this.setupStations();
    this.buildControlsAndCharts();
    this.rebuildAllCharts();
  }

  setupStations() {
    for (const data of this.data)
      for (const station of data.stations)
        this.stations[station.id] = station;
  }

  rebuildAllCharts() {
    for (const unit in kKnownUnits)
      this.rebuildChartForUnit(unit);
  }

  selectedYearChanged(e) {
    this.rebuildAllCharts();
  }

  selectedStationChanged(e) {
    this.rebuildAllCharts();
  }

  buildControlsAndCharts() {
    const yearlyControls = document.createElement("yearly-controls");
    for (const data of this.data) {
      const label = document.createElement("label");
      const input = document.createElement("input");
      input.value = data.year;
      input.type = "checkbox";
      input.checked = true;
      input.addEventListener("change", e => this.selectedYearChanged(e));
      label.appendChild(input);
      label.appendChild(document.createTextNode(data.year));
      yearlyControls.appendChild(label);
    }

    const stationControls = document.createElement("station-controls");
    for (const [_, station] of Object.entries(this.stations)) {
      const label = document.createElement("label");
      const input = document.createElement("input");
      input.value = station.id;
      input.type = "checkbox";
      input.checked = true;
      input.addEventListener("change", e => this.selectedStationChanged(e));
      label.appendChild(input);
      label.appendChild(document.createTextNode(`${station.id} - ${station.name} (${station.city}, ${station.province})`));
      label.title = `${station.altitude} - ${station.latitude} - ${station.longitude}`;
      stationControls.appendChild(label);
    }

    this.controls.appendChild(yearlyControls);
    this.controls.appendChild(stationControls);

    for (const unit in kKnownUnits)
      this.buildControlsAndChartsForUnit(unit);
  }

  buildControlsAndChartsForUnit(unit) {
    const container = document.createElement("chart-container");
    this.charts[unit] = container;

    const controls = document.createElement("metric-controls")
    for (const m of kKnownUnits[unit]) {
      const metric = kKnownMetrics[m];
      const label = document.createElement("label");
      const input = document.createElement("input");
      input.value = m;
      input.type = "checkbox";
      input.checked = true;
      label.appendChild(input);
      input.addEventListener("change", e => this.selectedMetricChanged(e));
      label.appendChild(document.createTextNode(`${metric.pretty} (${metric.unit})`));
      controls.appendChild(label);
    }

    const chart = document.createElement("chart");

    container.appendChild(controls);
    container.appendChild(chart);

    this.rebuildChartForUnit(unit);
  }

  enabledYears() {
    const enabled = new Set();
    for (const input of this.controls.querySelector("yearly-controls").querySelectorAll("input"))
      if (input.checked)
        enabled.add(parseInt(input.value, 10));
    return enabled;
  }

  enabledStations() {
    const enabled = new Set();
    for (const input of this.controls.querySelector("station-controls").querySelectorAll("input"))
      if (input.checked)
        enabled.add(input.value);
    return enabled;
  }

  rebuildChartForUnit(unit) {
    const container = this.charts[unit];

    const chart = container.querySelector("chart");
    while (chart.lastChild)
      chart.lastChild.remove();

    const enabledYears = this.enabledYears();
    if (enabledYears.size == 0)
      return;

    const enabledStations = this.enabledStations();
    if (enabledStations.size == 0)
      return;

    const enabledMetrics = new Set();
    for (const input of container.querySelector("metric-controls").querySelectorAll("input"))
      if (input.checked)
        enabledMetrics.add(input.value);

    if (enabledMetrics.size == 0)
      return;

    let max = -Infinity;
    let min = Infinity;
    const lines = {};
    for (const data of this.data) {
      if (!enabledYears.has(data.year))
        continue;
      for (const m in data) {
        if (!enabledMetrics.has(m))
          continue;
        lines[m] = {};
        for (const yearData of data[m]) {
          const station = yearData.station_id;
          if (!enabledStations.has(station))
            continue;
          for (const month of kMonths) {
            const value = yearData[month];
            if (!value)
              continue;
            min = Math.min(min, value);
            max = Math.max(max, value);
            if (!lines[m][station])
              lines[m][station] = [];
            lines[m][station].push({
              year: data.year,
              month: month,
              value: value,
            });
          }
        }
      }
    }
    console.log(lines);
  }
};

})();

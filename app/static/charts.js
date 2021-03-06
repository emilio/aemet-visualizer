(function() {

const kSvgNs = "http://www.w3.org/2000/svg";

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


// TODO: Expand + tune
const kLineColors = [
  "red",
  "green",
  "blue",
  "yellow",
  "orange",
  "purple",
  "fuchsia",
];

const kModes = [
  {
    id: "full",
    pretty: "Show all years",
  },
  {
    id: "yearly",
    pretty: "Show overlaid years",
  },
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
  absolute_max_temperature: {
    pretty: "Absolute max temperature",
    unit: "Celsius",
    with_date: true,
  },
  absolute_min_temperature: {
    pretty: "Absolute min temperature",
    unit: "Celsius",
    with_date: true,
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
  max_rain: {
    pretty: "Max rain",
    unit: "Mm",
    with_date: true,
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
    pretty: "Average relative humidity",
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
  max_pressure: {
    pretty: "Max pressure",
    unit: "hPa",
    multiplier: 0.1,
    with_date: true,
  },
  min_pressure: {
    pretty: "Min pressure",
    unit: "hPa",
    multiplier: 0.1,
    with_date: true,
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

const kDefaultDotRadius = 2.5;
const kDefaultLineThickness = 1;

window.Charts = class Charts {
  constructor(chartContainer, controls, schema, afterFrameCallback) {
    this.chartContainer = chartContainer;
    this.controls = controls;
    this.schema = schema;
    this.data = {};
    this.loadingData = {};
    this.width = 1200;
    this.height = 600;
    this.axisPadding = {
      left: 60,
      bottom: 40,
      right: 60,
      top: 20,
    };
    this.afterFrameCallback = afterFrameCallback;
    this.stations = {};
    this.charts = {};
    this.combinedChart = null;
    this.scheduledFrameUpdates = new Set();
    this.combinedChartNeedsRebuild = false;
    this.animationFrame = 0;
    this.setupStations();
    this.buildControlsAndCharts();
    this.scheduleRebuildAllCharts();
  }

  dataFor(year, unit) {
    const cachedData = this.data[year];
    if (cachedData)
      return cachedData;
    if (!this.loadingData[year]) {
      const promise = fetch("static/data/" + year + ".json")
        .then(data => data.json())
        .then(data => {
          console.log("Successfully loaded data for " + year);
          this.loadedData(year, data);
        })
        .catch(error => {
          console.error("Failed to fetch data for ", year, error);
          this.loadedData(year, []);
        });

      this.loadingData[year] = {
        promise,
        units: new Set(),
      };
    }
    this.loadingData[year].units.add(unit);
    return null;
  }

  loadedData(year, data) {
    this.data[year] = data;
    for (const unit of this.loadingData[year].units)
      this.scheduleRebuildChartForUnit(unit);
    delete this.loadingData[year];
  }

  scheduleRebuildAllCharts() {
    for (const unit in kKnownUnits)
      this.scheduleRebuildChartForUnit(unit);
  }

  scheduleRebuildChartForUnit(unit) {
    this.scheduledFrameUpdates.add(unit);
    this.scheduleFrameUpdate();
  }

  scheduleCombinedChartRebuild() {
    this.combinedChartNeedsRebuild = true;
    this.scheduleFrameUpdate();
  }

  scheduleFrameUpdate() {
    if (this.animationFrame)
      return;
    this.animationFrame = requestAnimationFrame(() => {
      for (const unit of this.scheduledFrameUpdates)
        this.rebuildChartForUnit(unit);
      this.maybeRebuildCombinedChart(this.scheduledFrameUpdates, this.combinedChartNeedsRebuild);
      if (this.afterFrameCallback)
        this.afterFrameCallback(this);
      this.scheduledFrameUpdates = new Set();
      this.combinedChartNeedsRebuild = false;
      this.animationFrame = 0;
    });
  }

  setupStations() {
    for (const data of this.schema)
      for (const station of data.stations)
        this.stations[station.id] = station;
  }

  checkboxChanged(input) {
    if (input.checked)
      input.parentNode.classList.add("checked")
    else
      input.parentNode.classList.remove("checked")
  }

  selectedModeChanged(input) {
    document.documentElement.setAttribute("data-selected-mode", input.value);
    // Event is only fired on the new (checked) checkbox.
    for (const label of input.form.querySelectorAll("label"))
      if (label.firstChild.checked)
        label.classList.add("checked");
      else
        label.classList.remove("checked");
    this.scheduleRebuildAllCharts();
  }

  selectedYearChanged(input) {
    this.checkboxChanged(input);
    this.scheduleRebuildAllCharts();
  }

  enabledMetricChanged(input) {
    this.checkboxChanged(input);
    for (const unit in kKnownUnits)
      if (kKnownUnits[unit].includes(input.value))
        this.scheduleRebuildChartForUnit(unit);
  }

  selectedStationChanged(input) {
    this.checkboxChanged(input);
    this.scheduleRebuildAllCharts();
  }

  buildControlsAndCharts() {
    const yearlyControls = document.createElement("yearly-controls");
    const perAggregateData = {};

    const addYearlyControl = (parentNode, data) => {
      const label = document.createElement("label");
      const input = document.createElement("input");
      input.value = data.year;
      input.type = "checkbox";
      input.checked = !data.is_aggregate;
      input.addEventListener("change", e => this.selectedYearChanged(e.target));
      label.appendChild(input);
      if (input.checked)
        label.classList.add("checked");
      label.appendChild(document.createTextNode(data.year));
      parentNode.appendChild(label);
    };

    for (const data of this.schema) {
      if (!data.is_aggregate) {
        addYearlyControl(yearlyControls, data);
        continue;
      }
      if (!perAggregateData[data.is_aggregate])
        perAggregateData[data.is_aggregate] = [];
      perAggregateData[data.is_aggregate].push(data);
    }

    for (const aggregate in perAggregateData) {
      const aggregateControls = document.createElement("yearly-aggregate-controls");
      aggregateControls.setAttribute("data-aggregate", aggregate);
      for (const data of perAggregateData[aggregate])
        addYearlyControl(aggregateControls, data);
      yearlyControls.appendChild(aggregateControls);
    }

    const modeControls = document.createElement("mode-controls");
    {
      const form = document.createElement("form"); // To ensure the name doesn't escape.
      let modeChecked = true;
      for (const mode of kModes) {
        const label = document.createElement("label");
        const input = document.createElement("input");
        input.value = mode.id;
        input.type = "radio";
        input.name = "mode";
        if (modeChecked) {
          document.documentElement.setAttribute("data-selected-mode", input.value);
          label.classList.add("checked");
          input.checked = true;
          modeChecked = false;
        }
        input.addEventListener("change", e => this.selectedModeChanged(e.target));
        label.appendChild(input);
        label.appendChild(document.createTextNode(mode.pretty));
        form.appendChild(label);
      }
      modeControls.appendChild(form);
    }

    const stationControls = document.createElement("station-controls");
    {
      const button = this.buildToggleAllButton(stationControls, input => this.selectedStationChanged(input));
      stationControls.appendChild(button);

      for (const [_, station] of Object.entries(this.stations)) {
        const label = document.createElement("label");
        const input = document.createElement("input");
        input.value = station.id;
        input.type = "checkbox";
        input.checked = true;
        input.addEventListener("change", e => this.selectedStationChanged(e.target));
        label.classList.add("checked");
        label.appendChild(input);
        label.appendChild(document.createTextNode(`${station.id} - ${station.name} (${station.city}, ${station.province})`));
        label.title = `${station.altitude} - ${station.latitude} - ${station.longitude}`;
        stationControls.appendChild(label);
      }
    }

    const styleControls = document.createElement("style-controls");
    {
      const createNumberControl = (name, labelText, defaultValue) => {
        const input = document.createElement("input");
        input.name = name;
        input.type = "number";
        input.step = "0.1"
        input.value = defaultValue;
        input.addEventListener("blur", () => this.scheduleRebuildAllCharts());
        const label = document.createElement("label");
        label.appendChild(document.createTextNode(labelText));
        label.appendChild(input);
        return label;
      }

      const thickness = createNumberControl("line-thickness", "Line thickness", kDefaultLineThickness);
      const dotRadius = createNumberControl("dot-radius", "Dot radius", kDefaultDotRadius);
      const form = document.createElement("form");
      form.addEventListener("submit", e => {
        e.preventDefault();
        this.scheduleRebuildAllCharts();
      });
      form.appendChild(thickness);
      form.appendChild(dotRadius);
      styleControls.appendChild(form);
    }

    this.controls.appendChild(yearlyControls);
    this.controls.appendChild(modeControls);
    this.controls.appendChild(stationControls);
    this.controls.appendChild(styleControls);

    for (const unit in kKnownUnits)
      this.buildControlsAndChartsForUnit(unit);
    this.buildCombinedChart();

    const titlePopup = document.createElement("title-popup");
    titlePopup.style.display = "none";
    titlePopup.style.opacity = "0";
    this.chartContainer.appendChild(titlePopup);

    let timeoutId = null;
    this.chartContainer.addEventListener("mousemove", e => {
      if (!e.target.hasAttribute("title")) {
        if (!timeoutId) {
          timeoutId = setTimeout(function() {
            timeoutId = null;
            titlePopup.style.opacity = "0";
            requestAnimationFrame(() => {
              requestAnimationFrame(() => {
                if (titlePopup.style.opacity == "0")
                  titlePopup.style.display = "none";
              })
            });
          }, 800);
        }
        return;
      }
      if (timeoutId) {
        clearTimeout(timeoutId);
        timeoutId = null;
      }
      const title = e.target.getAttribute("title");
      if (titlePopup.style.display !== "none" && titlePopup.textContent == title)
        return;
      titlePopup.style.display = "block";
      titlePopup.textContent = title;
      titlePopup.style.left = e.clientX + "px";
      titlePopup.style.top = e.clientY + "px";
      getComputedStyle(titlePopup).display; // Trigger the opacity transition below.
      titlePopup.style.opacity = "1";
    });
  }

  buildToggleAllButton(container, onChange) {
    const button = document.createElement("button");
    button.appendChild(document.createTextNode("Select / unselect all"));
    button.addEventListener("click", () => {
      let checked = false;
      let checkedSet = false;
      for (const input of container.querySelectorAll("input")) {
        if (!checkedSet)
          checked = !input.checked;
        checkedSet = true;
        const changed = checked != input.checked;
        input.checked = checked;
        if (changed)
          onChange(input);
      }
    }, false);
    return button;
  }

  buildControlsAndChartsForUnit(unit) {
    const container = document.createElement("chart-container");
    this.charts[unit] = container;

    const controls = document.createElement("metric-controls")
    const button = this.buildToggleAllButton(controls, input => this.enabledMetricChanged(input));
    controls.appendChild(button);

    for (const m of kKnownUnits[unit]) {
      const metric = kKnownMetrics[m];
      const label = document.createElement("label");
      const input = document.createElement("input");
      input.value = m;
      input.type = "checkbox";
      input.checked = true;
      label.appendChild(input);
      label.classList.add("checked");
      input.addEventListener("change", e => this.enabledMetricChanged(e.target));
      label.appendChild(document.createTextNode(`${metric.pretty} (${metric.unit})`));
      controls.appendChild(label);
    }

    const chart = document.createElement("chart");

    container.appendChild(controls);
    container.appendChild(chart);

    this.chartContainer.appendChild(container);
    this.scheduleRebuildChartForUnit(unit);
  }

  combinedChartUnits() {
    const left = this.combinedChart.querySelector("select[name=left-unit]");
    const right = this.combinedChart.querySelector("select[name=right-unit]");
    return [left.options[left.selectedIndex].value, right.options[right.selectedIndex].value];
  }

  setStateFromOptions(select, optionValueToSelect, onChange) {
    for (const option of select.options) {
      const selected = option.value == optionValueToSelect;
      if (option.selected == selected)
        continue;
      option.selected = selected;
      onChange(option);
    }
  }

  setCombinedChartUnits(units) {
    if (!units || units.length != 2)
      return;
    const left = this.combinedChart.querySelector("select[name=left-unit]");
    const right = this.combinedChart.querySelector("select[name=right-unit]");

    this.setStateFromOptions(left, units[0], () => this.scheduleCombinedChartRebuild());
    this.setStateFromOptions(right, units[1], () => this.scheduleCombinedChartRebuild());
  }

  setCombinedChartRatio(value) {
    if (!value || this.combinedChartRatio() == value)
      return;
    const ratio = this.combinedChart.querySelector("input[name=ratio]");
    ratio.value = value;
    this.scheduleCombinedChartRebuild();
  }

  combinedChartRatio() {
    return this.combinedChart.querySelector("input[name=ratio]").valueAsNumber || 1.0;
  }

  maybeRebuildCombinedChart(unitsToRebuild, forceRebuild) {
    const [leftUnit, rightUnit] = this.combinedChartUnits();
    if (!forceRebuild && !unitsToRebuild.has(leftUnit) && !unitsToRebuild.has(rightUnit))
      return;
    const chart = this.combinedChart.querySelector("chart");
    while (chart.lastChild)
      chart.lastChild.remove();
    if (!leftUnit || !rightUnit)
      return;
    const ratio = this.combinedChartRatio();
    this.rebuildChartForUnit(leftUnit, rightUnit, ratio);
  }

  buildCombinedChart() {
    const container = document.createElement("chart-container");
    this.combinedChart = container;

    const controls = document.createElement("combined-chart-controls");
    const form = document.createElement("form");
    form.addEventListener("submit", e => {
      this.scheduleCombinedChartRebuild();
      e.preventDefault();
    });
    const createSelect = (name, labelText) => {
      const select = document.createElement("select");
      select.addEventListener("change", () => this.scheduleCombinedChartRebuild());
      select.name = name;
      {
        const emptyOption = document.createElement("option");
        emptyOption.value = "";
        emptyOption.appendChild(document.createTextNode(" -- select -- "));
        select.appendChild(emptyOption);
      }
      for (const unit in kKnownUnits) {
        const option = document.createElement("option");
        option.value = unit;
        option.appendChild(document.createTextNode(unit));
        select.appendChild(option);
      }

      const label = document.createElement("label");
      label.appendChild(document.createTextNode(labelText));
      label.appendChild(select);
      return label;
    };

    // TODO(emilio): Add style controls? Bar vs. dots or what not.
    form.appendChild(createSelect("left-unit", "Left unit"));
    form.appendChild(createSelect("right-unit", "Right unit"));
    form.appendChild((() => {
      const ratio = document.createElement("input");
      ratio.type = "number";
      ratio.name = "ratio";
      ratio.value = "1";
      ratio.addEventListener("blur", () => this.scheduleCombinedChartRebuild());

      const label = document.createElement("label");
      label.appendChild(document.createTextNode("Ratio"));
      label.appendChild(ratio);
      return label;
    })());

    controls.appendChild(form);
    container.appendChild(controls);

    const chart = document.createElement("chart");
    container.appendChild(chart);
    this.chartContainer.appendChild(container);
  }

  controlInputs(kind) {
    return this.controls.querySelector(kind).querySelectorAll("input");
  }

  setStateFromArray(inputs, values, onChange) {
    for (const input of inputs) {
      let checked = false;
      // NOTE(emilio): Not using includes() to get != rather than !== semantics.
      for (const value of values) {
        if (value != input.value)
          continue;
        checked = true;
        break;
      }
      if (input.checked == checked)
        continue;
      input.checked = checked;
      onChange(input);
    }
  }

  collectEnabledControls(controlsName) {
    const enabled = new Set();
    for (const input of this.controlInputs(controlsName))
      if (input.checked)
        enabled.add(input.value);
    return enabled;
  }

  enabledYears() {
    return this.collectEnabledControls("yearly-controls");
  }

  enabledYearsAccountingForMode() {
    const enabled = this.enabledYears();
    if (!this.overlayYears()) {
      for (const data of this.schema) {
        if (data.is_aggregate)
          enabled.delete(data.year);
      }
    }
    return enabled;
  }

  setEnabledYears(years) {
    this.setStateFromArray(this.controlInputs("yearly-controls"), years, input => this.selectedYearChanged(input))
  }

  enabledStations() {
    return this.collectEnabledControls("station-controls");
  }

  setEnabledStations(stations) {
    this.setStateFromArray(this.controlInputs("station-controls"), stations, input => this.selectedStationChanged(input))
  }

  enabledMetrics(unit) {
    const enabledMetrics = new Set();
    for (const input of this.charts[unit].querySelector("metric-controls").querySelectorAll("input"))
      if (input.checked)
        enabledMetrics.add(input.value);
    return enabledMetrics;
  }

  setEnabledMetrics(unit, metrics) {
    if (!this.charts[unit])
      return;
    this.setStateFromArray(this.charts[unit].querySelector("metric-controls").querySelectorAll("input"), metrics, input => this.enabledMetricChanged(input));
  }

  selectedMode() {
    for (const input of this.controlInputs("mode-controls"))
      if (input.checked)
        return input.value;
    return kModes[0].id;
  }

  overlayYears() {
    return this.selectedMode() === "yearly";
  }

  setSelectedMode(mode) {
    for (const input of this.controlInputs("mode-controls")) {
      if (input.value != mode)
        continue;
      if (!input.checked) {
        input.checked = true;
        this.selectedModeChanged(input);
      }
      return;
    }
  }

  dotRadiusInput() {
    return this.controls.querySelector("style-controls").querySelector("input[name=dot-radius]");
  }

  dotRadius() {
    return this.dotRadiusInput().valueAsNumber || kDefaultDotRadius;
  }

  setDotRadius(radius) {
    if (radius === undefined)
      radius = kDefaultDotRadius;
    if (this.dotRadius() == radius)
      return;
    this.dotRadiusInput().value = radius;
    this.scheduleRebuildAllCharts();
  }

  lineThicknessInput() {
    return this.controls.querySelector("style-controls").querySelector("input[name=line-thickness]");
  }

  lineThickness() {
    return this.lineThicknessInput().valueAsNumber || kDefaultLineThickness;
  }

  setLineThickness(thickness) {
    if (thickness === undefined)
      thickness = kDefaultLineThickness;
    if (this.lineThickness() == thickness)
      return;
    this.lineThicknessInput().value = thickness;
    this.scheduleRebuildAllCharts();
  }

  linesForUnit(unit, enabledYears, enabledStations, overlayYears) {
    let currentYear = 0;
    const enabledMetrics = this.enabledMetrics(unit);
    if (enabledMetrics.size == 0)
      return;

    let loading = false;
    let min = +Infinity;
    let max = -Infinity;
    const lines = {};

    if (unit == "%") {
      min = 0;
      max = 100;
    }

    for (const schemaEntry of this.schema) {
      if (!enabledYears.has(schemaEntry.year))
        continue;
      const data = this.dataFor(schemaEntry.year, unit);
      if (!data) {
        loading = true;
        continue;
      }
      for (const m in data) {
        if (!enabledMetrics.has(m))
          continue;
        for (const yearData of data[m]) {
          const station = yearData.station_id;
          if (!enabledStations.has(station))
            continue;
          for (let currentMonth = 0; currentMonth < kMonths.length; ++currentMonth) {
            const month = kMonths[currentMonth];
            let longValue = yearData[month];
            if (longValue === null)
              continue;
            // longValue should either be a number or a WithDate<>.
            let value = kKnownMetrics[m].with_date ? longValue.value : longValue;
            if (kKnownMetrics[m].multiplier)
              value *= kKnownMetrics[m].multiplier;
            let key = `${kKnownMetrics[m].pretty} - ${this.stations[station].name}`;
            if (overlayYears)
              key += ` - ${data.year}`;

            min = Math.min(min, value);
            max = Math.max(max, value);

            if (!lines[key])
              lines[key] = [];
            lines[key].push({
              year: overlayYears ? 0 : currentYear,
              month: currentMonth,
              value,
              date: longValue.date,
            });
          }
        }
      }
      currentYear++;
    }
    return { lines, min, max, loading };
  }

  downloadChart(e) {
    const data = e.target.parentNode.querySelector("svg").outerHTML;
    const svg = new Blob([data], {type: "image/svg+xml;charset=utf-8"});
    const url = URL.createObjectURL(svg);
    e.target.href = url;
    setTimeout(() => URL.revokeObjectURL(url), 0);
  }

  rebuildChartForUnit(unit, unit2 = undefined, unit2Scale = 1.0) {
    const container = unit2 ? this.combinedChart : this.charts[unit];

    const chart = container.querySelector("chart");
    while (chart.lastChild)
      chart.lastChild.remove();

    const enabledYears = this.enabledYearsAccountingForMode();
    if (enabledYears.size == 0)
      return;

    const enabledStations = this.enabledStations();
    if (enabledStations.size == 0)
      return;

    let loading = false;

    const overlayYears = this.overlayYears();

    const unitMetadata = this.linesForUnit(unit, enabledYears, enabledStations, overlayYears);
    let unit2Metadata = undefined;
    if (unit2)
      unit2Metadata = this.linesForUnit(unit2, enabledYears, enabledStations, overlayYears);

    if (!unitMetadata || (unit2 && !unit2Metadata))
      return;

    if (unitMetadata.loading || (unit2Metadata && unit2Metadata.loading))
      chart.classList.add("loading");
    else
      chart.classList.remove("loading");

    // If we're displaying two units, we need at least one common point, which
    // is going to be the zero, and we need to adjust the scale so that the max
    // and min end up lined up with the zero being the same in both scales.
    if (unit2) {
      for (const meta of [unitMetadata, unit2Metadata]) {
        meta.min = Math.min(0, meta.min);
        meta.max = Math.max(0, meta.max);
      }
      unitMetadata.max = Math.max(unitMetadata.max, unit2Metadata.max / unit2Scale);
      unitMetadata.min = Math.min(unitMetadata.min, unit2Metadata.min / unit2Scale);
      unit2Metadata.max = Math.max(unit2Metadata.max, unitMetadata.max * unit2Scale);
      unit2Metadata.min = Math.min(unit2Metadata.min, unitMetadata.min * unit2Scale);
    }

    const createDownloadButton = (text, download) => {
      const button = document.createElement("a");
      button.classList.add("download-button");
      button.href = "#";
      button.target = "_blank";
      if (download)
        button.download = download;
      button.appendChild(document.createTextNode(text));
      button.addEventListener("click", (e) => this.downloadChart(e));
      return button;
    };

    chart.appendChild(createDownloadButton("Open in new tab"));
    chart.appendChild(createDownloadButton("Download", "graph.svg"));

    const svg = document.createElementNS(kSvgNs, "svg");
    svg.setAttribute("font-size", "12px");
    svg.setAttribute("font-family", "sans");
    svg.setAttribute("viewbox", "0 0 " + this.width + " " + this.height);
    svg.setAttribute("xmlns", "http://www.w3.org/2000/svg");
    svg.setAttribute("width", this.width);
    svg.setAttribute("height", this.height);

    const createLine = function() {
      const line = document.createElementNS(kSvgNs, "line");
      line.setAttribute("stroke-width", "1px");
      line.setAttribute("stroke", "lightgrey");
      return line;
    }

    // Add two axes.
    {
      const xAxis = document.createElementNS(kSvgNs, "g");
      xAxis.classList.add("x-axis");

      {
        const xLine = createLine();
        xLine.setAttribute("x1", this.axisPadding.left)
        xLine.setAttribute("x2", this.width - this.axisPadding.right)

        xLine.setAttribute("y1", this.height - this.axisPadding.bottom)
        xLine.setAttribute("y2", this.height - this.axisPadding.bottom)

        xAxis.appendChild(xLine);
        svg.appendChild(xAxis);
      }

      {
        const yAxis = document.createElementNS(kSvgNs, "g");
        yAxis.classList.add("y-axis");

        const yLine = createLine();
        yLine.setAttribute("x1", this.axisPadding.left)
        yLine.setAttribute("x2", this.axisPadding.left)

        yLine.setAttribute("y1", this.axisPadding.top);
        yLine.setAttribute("y2", this.height - this.axisPadding.bottom)
        yAxis.appendChild(yLine);
        svg.appendChild(yAxis);
      }


      if (unit2) {
        const yAxis = document.createElementNS(kSvgNs, "g");
        yAxis.classList.add("y-axis");
        yAxis.classList.add("y-axis-second-unit");

        const yLine = createLine();
        yLine.setAttribute("x1", this.width - this.axisPadding.right)
        yLine.setAttribute("x2", this.width - this.axisPadding.right)

        yLine.setAttribute("y1", this.axisPadding.top);
        yLine.setAttribute("y2", this.height - this.axisPadding.bottom)
        yAxis.appendChild(yLine);
        svg.appendChild(yAxis);
      }
    }

    const availWidth = this.width - this.axisPadding.left - this.axisPadding.right;
    const availHeight = this.height - this.axisPadding.top - this.axisPadding.bottom;

    const monthCount = overlayYears ? kMonths.length : enabledYears.size * kMonths.length;
    const sizePerMonth = availWidth / (monthCount - 1);

    {
      // The x axis labels.
      const labels = document.createElementNS(kSvgNs, "g");
      labels.classList.add("x-labels");

      let consumedSize = this.axisPadding.left;

      let appendLabel = content => {
        const text = document.createElementNS(kSvgNs, "text");
        text.setAttribute("text-anchor", "middle");
        text.setAttribute("dominant-baseline", "middle");
        text.setAttribute("y", this.height - this.axisPadding.bottom / 2);
        text.setAttribute("x", consumedSize);
        text.appendChild(document.createTextNode(content));
        labels.appendChild(text);
        consumedSize += sizePerMonth;
      };

      let appendMonthLabels = year => {
        for (let i = 0; i < kMonths.length; ++i)
          appendLabel(year && i == 0 ? year : kMonths[i].substring(0, 3));
      };

      // XXX: Is Set iteration order well-defined enough?
      // XXX: What if years are not contiguous? What does that even mean?
      if (overlayYears)
        appendMonthLabels(null);
      else {
        for (const year of enabledYears)
          appendMonthLabels(year);
      }
      svg.appendChild(labels);
    }

    let lineIndex = 0;
    for (const meta of [unitMetadata, unit2Metadata]) {
      if (!meta)
        continue;

      const kVerticalAxisPoints = 11; // TODO(emilio): Allow tuning?
      const {min, max, lines} = meta;
      console.log(meta);
      const yRange = max - min;

      const labels = document.createElementNS(kSvgNs, "g");
      labels.classList.add("y-labels");
      let x = 0;
      if (meta == unit2Metadata) {
        x = this.width - this.axisPadding.right;
        labels.classList.add("y-labels-second-unit");
      }

      const availableVerticalSize = this.height - this.axisPadding.bottom - this.axisPadding.top;
      const verticalIncrement = availableVerticalSize / (kVerticalAxisPoints - 1);

      let consumedVerticalSize = this.axisPadding.top;
      let sawZero = false;
      const createYAxisLabel = function() {
        const label = document.createElementNS(kSvgNs, "text");
        label.setAttribute("text-anchor", "start");
        label.setAttribute("dominant-baseline", "middle");
        return label;
      };
      for (let i = 0; i < kVerticalAxisPoints; i++) {
        const percentage = i / (kVerticalAxisPoints - 1);
        const label = createYAxisLabel();
        const value = min + yRange * (1.0 - percentage);

        // TODO(emilio): Perhaps we should also skip showing the current point
        // if it's very close to zero.
        if (value == 0)
          sawZero = true;

        label.setAttribute("x", x);
        label.setAttribute("y", consumedVerticalSize);
        label.appendChild(document.createTextNode(value.toFixed(2)));
        labels.appendChild(label);

        consumedVerticalSize += verticalIncrement;
      }

      // We want to always display the zero if comparing.
      if (unit2 && !sawZero) {
        const zero = ((yRange - max) / yRange);
        const label = createYAxisLabel();
        label.setAttribute("x", x);
        label.setAttribute("y", (1.0 - zero) * availableVerticalSize + this.axisPadding.top);
        label.appendChild(document.createTextNode("0"));
        labels.appendChild(label);
      }

      svg.appendChild(labels);

      const pointContainer = document.createElementNS(kSvgNs, "g");
      pointContainer.classList.add("points");

      const dotRadius = this.dotRadius();
      const lineThickness = this.lineThickness();
      for (const lineKey in lines) {
        const points = lines[lineKey];
        const line = document.createElementNS(kSvgNs, "polyline");
        const pointsAttr = [];
        const color = kLineColors[lineIndex++ % kLineColors.length];
        for (const point of points) {
          const circle = document.createElementNS(kSvgNs, "circle");
          const x = this.axisPadding.left + (point.year * kMonths.length + point.month) * sizePerMonth;
          const y = this.axisPadding.top + (1 - ((point.value - min) / yRange)) * availHeight;
          circle.setAttribute("fill", color);
          circle.setAttribute("stroke", "none");
          circle.setAttribute("cx", x);
          circle.setAttribute("cy", y);
          circle.setAttribute("r", dotRadius);
          let title = `${lineKey} - ${kMonths[point.month]} - ${point.value}`;
          if (point.date)
            title += ` - (date: ${point.date})`;
          circle.setAttribute("title", title);
          pointsAttr.push(`${x},${y}`);
          pointContainer.appendChild(circle);
        }
        line.setAttribute("points", pointsAttr.join(" "));
        line.setAttribute("title", lineKey);
        line.setAttribute("stroke", color);
        line.setAttribute("stroke-width", lineThickness);
        line.setAttribute("fill", "none");
        pointContainer.insertBefore(line, pointContainer.firstChild);
      }
      svg.appendChild(pointContainer);
    }
    chart.appendChild(svg);
  }

  state() {
    return {
      enabledYears: Array.from(this.enabledYears()),
      enabledStations: Array.from(this.enabledStations()),
      mode: this.selectedMode(),
      style: {
        lineThickness: this.lineThickness(),
        dotRadius: this.dotRadius(),
      },
      units: (() => {
        const units = {};
        for (const unit in kKnownUnits)
          units[unit] = Array.from(this.enabledMetrics(unit));
        return units;
      })(),
      combined: {
        units: this.combinedChartUnits(),
        ratio: this.combinedChartRatio(),
      },
    };
  }

  setState(state) {
    if (state.mode)
      this.setSelectedMode(state.mode);
    if (state.style) {
      this.setLineThickness(state.style.lineThickness);
      this.setDotRadius(state.style.dotRadius);
    }
    if (Array.isArray(state.enabledYears))
      this.setEnabledYears(state.enabledYears);
    if (Array.isArray(state.enabledStations))
      this.setEnabledStations(state.enabledStations);
    if (state.units)
      for (const unit in state.units)
        this.setEnabledMetrics(unit, state.units[unit]);
    if (state.combined) {
      this.setCombinedChartUnits(state.combined.units);
      this.setCombinedChartRatio(state.combined.ratio);
    }
  }
};

})();

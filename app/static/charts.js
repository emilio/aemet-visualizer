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

window.Charts = class Charts {
  constructor(chartContainer, controls, data, afterFrameCallback) {
    this.chartContainer = chartContainer;
    this.controls = controls;
    this.data = data;
    this.width = 1200;
    this.height = 600;
    this.axisPadding = {
      left: 40,
      bottom: 40,
      right: 0,
      top: 20,
    };
    this.afterFrameCallback = afterFrameCallback;
    this.dotSize = 5;
    this.stations = {};
    this.charts = {};
    this.scheduledFrameUpdates = new Set();
    this.animationFrame = 0;
    this.setupStations();
    this.buildControlsAndCharts();
    this.scheduleRebuildAllCharts();
  }

  scheduleRebuildAllCharts() {
    for (const unit in kKnownUnits)
      this.scheduleRebuildChartForUnit(unit);
  }

  scheduleRebuildChartForUnit(unit) {
    this.scheduledFrameUpdates.add(unit);
    if (this.animationFrame)
      return;
    this.animationFrame = requestAnimationFrame(() => {
      for (const unit of this.scheduledFrameUpdates)
        this.rebuildChartForUnit(unit);
      if (this.afterFrameCallback)
        this.afterFrameCallback(this);
      this.scheduledFrameUpdates = new Set();
      this.animationFrame = 0;
    });
  }

  setupStations() {
    for (const data of this.data)
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
    for (const data of this.data) {
      const label = document.createElement("label");
      const input = document.createElement("input");
      input.value = data.year;
      input.type = "checkbox";
      input.checked = true;
      input.addEventListener("change", e => this.selectedYearChanged(e.target));
      label.appendChild(input);
      label.classList.add("checked");
      label.appendChild(document.createTextNode(data.year));
      yearlyControls.appendChild(label);
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

    this.controls.appendChild(yearlyControls);
    this.controls.appendChild(modeControls);
    this.controls.appendChild(stationControls);

    for (const unit in kKnownUnits)
      this.buildControlsAndChartsForUnit(unit);

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

  enabledYears() {
    const enabled = new Set();
    for (const input of this.controlInputs("yearly-controls"))
      if (input.checked)
        enabled.add(parseInt(input.value, 10));
    return enabled;
  }

  setEnabledYears(years) {
    this.setStateFromArray(this.controlInputs("yearly-controls"), years, input => this.selectedYearChanged(input))
  }

  enabledStations() {
    const enabled = new Set();
    for (const input of this.controlInputs("station-controls"))
      if (input.checked)
        enabled.add(input.value);
    return enabled;
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

    const enabledMetrics = this.enabledMetrics(unit);
    if (enabledMetrics.size == 0)
      return;

    let max = -Infinity;
    let min = Infinity;

    // Special-case for percentages.
    if (unit == "%") {
      min = 0;
      max = 100;
    }

    const overlayYears = this.selectedMode() === "yearly";
    const lines = {};
    {
      let currentYear = 0;
      for (const data of this.data) {
        if (!enabledYears.has(data.year))
          continue;
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
              if (!longValue)
                continue;
              // longValue should either be a number or a WithDate<>.
              let value = kKnownMetrics[m].with_date ? longValue.value : longValue;
              if (kKnownMetrics[m].multiplier)
                value *= kKnownMetrics[m].multiplier;
              min = Math.min(min, value);
              max = Math.max(max, value);
              let key = `${kKnownMetrics[m].pretty} - ${this.stations[station].name}`;
              if (overlayYears)
                key += ` - ${data.year}`;

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
    }

    const svg = document.createElementNS(kSvgNs, "svg");
    svg.setAttribute("width", this.width);
    svg.setAttribute("height", this.height);

    // Add two axes.
    {
      const xAxis = document.createElementNS(kSvgNs, "g");
      xAxis.classList.add("x-axis");

      const xLine = document.createElementNS(kSvgNs, "line");
      xLine.setAttribute("x1", this.axisPadding.left)
      xLine.setAttribute("x2", this.width - this.axisPadding.right)

      xLine.setAttribute("y1", this.height - this.axisPadding.bottom)
      xLine.setAttribute("y2", this.height - this.axisPadding.bottom)

      xAxis.appendChild(xLine);

      const yAxis = document.createElementNS(kSvgNs, "g");
      yAxis.classList.add("y-axis");

      const yLine = document.createElementNS(kSvgNs, "line");
      yLine.setAttribute("x1", this.axisPadding.left)
      yLine.setAttribute("x2", this.axisPadding.left)

      yLine.setAttribute("y1", this.axisPadding.top);
      yLine.setAttribute("y2", this.height - this.axisPadding.bottom)

      yAxis.appendChild(yLine);

      svg.appendChild(xAxis);
      svg.appendChild(yAxis);
    }

    const availWidth = this.width - this.axisPadding.left - this.axisPadding.right;
    const availHeight = this.height - this.axisPadding.top - this.axisPadding.bottom;
    const yRange = max - min;

    const sizePerYear = overlayYears ? availWidth : availWidth / enabledYears.size;
    const sizePerMonth = sizePerYear / kMonths.length;

    {
      // The x axis labels.
      const labels = document.createElementNS(kSvgNs, "g");
      labels.classList.add("x-labels");

      let consumedSize = this.axisPadding.left;

      let appendLabel = content => {
        const text = document.createElementNS(kSvgNs, "text");
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

    {
      const labels = document.createElementNS(kSvgNs, "g");
      labels.classList.add("y-labels");

      const maxLabel = document.createElementNS(kSvgNs, "text");
      maxLabel.setAttribute("y", this.axisPadding.top);
      maxLabel.setAttribute("x", 0);
      maxLabel.appendChild(document.createTextNode(max.toFixed(2)));
      labels.appendChild(maxLabel);

      const minLabel = document.createElementNS(kSvgNs, "text");
      minLabel.setAttribute("y", this.height - this.axisPadding.bottom);
      minLabel.setAttribute("x", 0);
      minLabel.appendChild(document.createTextNode(min.toFixed(2)));
      labels.appendChild(minLabel);

      // TODO: y axis labels in the middle?
      svg.appendChild(labels);
    }

    {
      const pointContainer = document.createElementNS(kSvgNs, "g");
      pointContainer.classList.add("points");

      let lineIndex = 0;
      for (const lineKey in lines) {
        const points = lines[lineKey];
        const line = document.createElementNS(kSvgNs, "polyline");
        const pointsAttr = [];
        const color = kLineColors[lineIndex++ % kLineColors.length];
        for (const point of points) {
          const circle = document.createElementNS(kSvgNs, "circle");
          const x = this.axisPadding.left + point.year * sizePerYear + point.month * sizePerMonth;
          const y = this.axisPadding.top + (1 - ((point.value - min) / yRange)) * availHeight;
          circle.style.color = color;
          circle.setAttribute("cx", x);
          circle.setAttribute("cy", y);
          circle.setAttribute("r", this.dotSize / 2);
          let title = `${lineKey} - ${kMonths[point.month]} - ${point.value}`;
          if (point.date)
            title += ` - (date: ${point.date})`;
          circle.setAttribute("title", title);
          pointsAttr.push(`${x},${y}`);
          pointContainer.appendChild(circle);
        }
        line.setAttribute("points", pointsAttr.join(" "));
        line.setAttribute("title", lineKey);
        line.style.color = color;
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
      units: (() => {
        const units = {};
        for (const unit in kKnownUnits)
          units[unit] = Array.from(this.enabledMetrics(unit));
        return units;
      })(),
    };
  }

  setState(state) {
    if (state.mode)
      this.setSelectedMode(state.mode);
    if (Array.isArray(state.enabledYears))
      this.setEnabledYears(state.enabledYears);
    if (Array.isArray(state.enabledStations))
      this.setEnabledStations(state.enabledStations);
    if (state.units)
      for (const unit in state.units)
        this.setEnabledMetrics(unit, state.units[unit]);
  }
};

})();

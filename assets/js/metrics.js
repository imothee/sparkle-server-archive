const Chart = require("chart.js");

const chartColors = [
  "rgb(255, 99, 132)", // red
  "rgb(255, 159, 64)", //orange
  "rgb(255, 205, 86)", //yellow
  "rgb(75, 192, 192)", // green
  "rgb(54, 162, 235)", // blue
  "rgb(153, 102, 255)", // purple
  "rgb(231,233,237)", // grey
];

const config = {
  credentials: "include",
  headers: {
    "Content-Type": "application/json",
  },
};

const validateResponse = (response) => {
  if (!response.ok) {
    if (response.status == 400 || response.status == 422) {
      return response.json().then((data) => {
        console.log(data);
        throw new ValidationResponseError("Invalid request", data.errors);
      });
    } else {
      throw new Error("HTTP status " + response.status);
    }
  }
  return response.json();
};

const getMetrics = (appId) => {
  return fetch(`/admin/apps/${appId}/metrics/summaries`, {
    ...config,
    headers: {
      ...config.headers,
    },
    method: "GET",
  }).then((response) => validateResponse(response));
};

const parseMetrics = (data) => {
  return data.summaries.reduce(
    (acc, val) => {
      // Push the label into the labels array in acc
      const startDate = new Date(val.date_start);
      const endDate = new Date(val.date_end);

      const date_label = `${startDate.toLocaleDateString(undefined, {
        day: "numeric",
        month: "numeric",
      })} - ${endDate.toLocaleDateString(undefined, {
        day: "numeric",
        month: "numeric",
      })}`;
      acc.labels.push(date_label);

      for (const [key, value] of Object.entries(val.counts)) {
        // Key is lang, value is {en: 10, jp: 10}
        if (!acc.data.hasOwnProperty(key)) {
          acc.data[key] = {};
        }
        if (key === "users") {
          (acc.data[key].users = acc.data[key].users || []).push(value);
        } else {
          for (const [label, count] of Object.entries(value)) {
            if (!acc.data[key].hasOwnProperty(label)) {
              // Push a number of zeroes equal to the number of labels we've already added -1
              acc.data[key][label] = Array(acc.labels.length - 1).fill(0);
            }
            acc.data[key][label].push(count);
          }
        }
      }

      return acc;
    },
    {
      labels: [],
      data: {},
    }
  );
};

const renderCharts = (data) => {
  const metrics = parseMetrics(data);

  for (const canvas of document.getElementsByTagName("canvas")) {
    const name = canvas.getAttribute("id");

    const datasets = Object.entries(metrics.data[name]).map(([k, v], i) => {
      return {
        label: k,
        data: v,
        fill: false,
        backgroundColor: chartColors[i % 7],
        borderColor: chartColors[i % 7],
      };
    });

    const options = {
      type: "line",
      data: {
        labels: metrics.labels,
        datasets: datasets,
      },
    };

    new Chart(canvas, options);
  }
};

window.addEventListener(
  "load",
  function () {
    const charts = document.getElementById("charts");
    if (!charts) {
      return;
    }

    const appId = charts.getAttribute("data-app-id");

    getMetrics(appId)
      .then((data) => {
        renderCharts(data);
      })
      .catch((err) => {
        console.log(err);
      });
  },
  false
);

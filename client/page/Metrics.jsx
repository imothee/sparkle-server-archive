import * as React from "react";
import * as Models from "../shared/Models.bs.js";
import { LineChart, Line, Area, AreaChart, XAxis, YAxis, Tooltip, Legend } from 'recharts';

const colors = [
  '#a6cee3',
  '#1f78b4',
  '#b2df8a',
  '#33a02c',
  '#fb9a99',
  '#e31a1c',
  '#fdbf6f',
  '#ff7f00',
  '#cab2d6',
  '#6a3d9a',
  '#666666',
  '#b15928'
];

export default function Metrics({appId}) {
  const [metrics, setMetrics] = React.useState({});

  const titleFor = function(key) {
    switch (key) {
      case 'profiles':
        return 'Unique Users';
      case 'app_versions':
        return 'App Versions';
      case 'cpu64bits':
        return '64 Bit';
      case 'ncpus':
        return 'Number CPU';
      case 'cpu_freq_mhzs':
        return 'CPU Frequencies';
      case 'cputypes':
        return 'CPU Types';
      case 'cpusubtypes':
        return 'CPU SubTypes';
      case 'models':
        return 'Mac Models';
      case 'ram_mbs':
        return 'Ram (mb)';
      case 'os_versions':
        return 'OS Versions';
      case 'langs':
        return 'Languages';
      default:
        return '?';
    }
  }

  const parseMetrics = function(data) {
    return data.metrics.reduce((acc, val) => {
      if (!acc.hasOwnProperty(val.profile_key)) {
        acc[val.profile_key] = [];
      }
      let i = acc[val.profile_key].findIndex((el) => el.name === val.date);
      if (i === -1) {
        var new_metric = {
          name: val.date,
        };
        new_metric[val.profile_value] = val.count;
        acc[val.profile_key].push(new_metric);
      } else {
        acc[val.profile_key][i][val.profile_value] = val.count;
      }
      return acc;
    }, {});
  }

  const getMetrics = function() {
    fetch("/api/apps/" + appId + "/metrics")
      .then(function (prim) {
        return prim.json();
      })
      .then(function (data) {
        const m = parseMetrics(Models.Decode.metricsResponse(data));
        setMetrics(m);
      });
  };

  React.useEffect((function () {
    getMetrics();
  }), []);

  const buildLineChart = function(key) {
    let data = metrics[key];

    if (data === undefined || data.count === 0) {
      return (<p>Error rendering chart for {key}</p>);
    }

    let lineKeys = Object.keys(data[0]).filter((e) => e !== "name");
    data.sort((a, b) => Date.parse(a.name) - Date.parse(b.name));

    return (
      <div>
        <h2 className="text-center text-large p-4">{titleFor(key)}</h2>
        <LineChart data={data} width={450} height={400}>
          <XAxis dataKey="name" />
          <YAxis />
          <Tooltip />
          {/*<Legend width={100} wrapperStyle={{ top: 40, right: 20, backgroundColor: '#f5f5f5', border: '1px solid #d5d5d5', borderRadius: 3, lineHeight: '40px' }} />*/}
          {lineKeys.map((k, i) => (
            <Line key={k} type="monotone" dataKey={k} stroke={colors[i]} />
          ))}
        </LineChart>
      </div>
    )
  }

  const buildPercentAreaChart = function(key) {
    let data = metrics[key];

    if (data === undefined || data.count === 0) {
      return (<p>Error rendering chart for {key}</p>);
    }
    let lineKeys = Object.keys(data[0]).filter((e) => e !== "name");
    data.sort((a, b) => Date.parse(a.name) - Date.parse(b.name));

    return (
      <div>
        <h2 className="text-center text-large p-4">{titleFor(key)}</h2>
        <AreaChart data={data} width={450} height={400}>
          <XAxis dataKey="name" />
          <YAxis />
          <Tooltip />
          {/*<Legend width={100} wrapperStyle={{ top: 40, right: 20, backgroundColor: '#f5f5f5', border: '1px solid #d5d5d5', borderRadius: 3, lineHeight: '40px' }} />*/}
          {lineKeys.map((k, i) => (
            <Area key={k} type="monotone" dataKey={k} stroke={colors[i]} fill={colors[i]} />
          ))}
        </AreaChart>
      </div>
    )
  }

  return (
    <div className="max-w-[75rem] mx-auto px-4 sm:px-6 lg:px-8 py-20">
      <div className="p-4 bg-white block flex-row border-b border-gray-200 lg:mt-1.5">
        <div className="mb-1 w-full">
          <h1 className="text-xl sm:text-2xl font-semibold text-gray-900">Metrics for </h1>
        </div>
        <div className="w-full grid grid-cols-2 gap-4">
          {buildLineChart("profiles")}
          {buildPercentAreaChart("app_versions")}
          {buildLineChart("cpu64bits")}
          {buildLineChart("ncpus")}
          {buildLineChart("cpu_freq_mhzs")}
          {buildLineChart("cputypes")}
          {buildLineChart("cpusubtypes")}
          {buildLineChart("models")}
          {buildLineChart("ram_mbs")}
          {buildLineChart("os_versions")}
          {buildLineChart("langs")}
        </div>
      </div>
    </div>
  );
}


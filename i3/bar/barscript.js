const exec = require('child_process').exec;
const os = require('os');

// Load extensions of builtin objects
require('./builtins');
let { readableMem, theme } = require('./utils');

const setColor = (color) => `%{F${color}}%{B${theme.BLACK}}`;

/**
 * @typedef {Function[]} AsyncFunctionPair
 *
 * @param {*} accumulator The starting accumulator value.
 * @param {AsyncFunctionPair[]} callbackList Pairs of functions and callbacks to execute.
 * @param {Function} aggregateCallback Called once all other callbacks finish.
 *
 * Combines a list of callbacks and executes them in order. Each element in
 * the list is called and its return value handed to the second item in
 * the "tuple". The callback is called with the accumulator first, and then
 * an unpacked list of return from the function.
 *
 * After the final callback in the list is called, `aggregateCallback` is
 * called with the accumulator's final value.
 */
const aggregate = (accumulator, callbackList, aggregateCallback) => {
  let runNext = (callbackUnit) => {
    callbackUnit[0]((...args) => {
      callbackUnit[1](accumulator, ...args);
      let next = callbackList.pop()
      if(next) runNext(next);
      else aggregateCallback(accumulator);
    })
  }

  // Pop element off the front and run it.
  runNext(callbackList.shift());
};

// Thin wrapper around `child_process.exec()`. Discards `err`.
const runCmd = (cmd, cb) => exec(cmd, (err, so, se) => { if(!err) return cb(so, se); });

let getWorkspaceLabel = (cb) => {
  runCmd('i3-msg -t get_outputs | sed \'s/.*"current_workspace":"\\([^"]*\\)".*/\\1/\'', (out) => {
    cb(out.replace('\n', ''));
  });
}

let getVolume = (cb) => {
  runCmd('amixer get \'Master\'', (out) => {
    let range = parseInt(out.match(/Playback \d+\s*-\s*\d+/g)[0].match(/\d+/g)[1]);
    let muted = out.match(/\[(?:on|off)\]/g)[0].indexOf('off') > -1;
    let left = parseInt(out.match(/Playback\s+\d+/g)[1].match(/\d+/g)[0]);
    let right = parseInt(out.match(/Playback\s+\d+/g)[2].match(/\d+/g)[0]);
    let leftPercent = Math.round((left / range) * 100);
    let rightPercent = Math.round((right / range) * 100);

    cb({
      range: range,
      leftAbsolute: left,
      rightAbsolute: right,
      left: leftPercent,
      right: rightPercent,
      muted: muted
    });
  });
}

let getCPUStats = (cb) => {
  let getTemps = (icb) => {
    runCmd("sensors | grep -oP 'Core.*?\\.0' | grep -o '+[0-9.]*'", (out) => {
      icb(out.split('\n').trimLast().map(e => parseInt(e)));
    });
  }

  let getUtilization = (icb) => {
    let cpus = os.cpus();

    let avgTimes = {};

    let cpuTimes = cpus.map((cpu) => {
      let times = cpu.times.map((k, v) => v / cpus.length);

      for(let type in cpu.times) {
        times[type] = cpu.times[type] / cpus.length;

        // Init average counters if they are not present.
        if(!avgTimes[type]) avgTimes[type] = 0;

        avgTimes[type] += cpu.times[type] / cpus.length;
      }

      icb(times);
    });
  };

  aggregate({}, [
    [getTemps, (cpuStats, temps) => {
      cpuStats.temps = temps;
      cpuStats.average_temp = temps.reduce((a, e) => a + e) / temps.length;
    }],
    [getUtilization, (cpuStats, util) => {
      cpuStats.utilization = util;
    }]
  ], (cpuStats) => cb(cpuStats))

};

let getMemoryStats = (cb) => {
  runCmd('free', (out) => {
    let memStats = { raw: {}, readable: {} };
    out.split('\n').trimFirst().trimLast().forEach((line) => {
      let name = line.split(':')[0];
      let categories = ['total', 'used', 'free', 'shared', 'cache', 'available'];

      let rawObj = categories.zipObject(line.split(':')[1].split(/\s+/).trimFirst()).map((k, v) => parseInt(v));
      let readableObj = rawObj.map((k, v) => readableMem(v));

      memStats.raw[name.toLowerCase()] = rawObj;
      memStats.readable[name.toLowerCase()] = readableObj;
    });

    cb(memStats);
  });
};

let getBatteryStats = (cb) => {
  runCmd('cat /sys/class/power_supply/BAT1/uevent', (out) => {
    let batteryStats = {};
    out.split('\n').reverse().slice(1).reverse().forEach((e) => {
      let key = e.split('=')[0];
      let val = e.split('=')[1];

      batteryStats[key] = parseFloat(val) || val;
    });
    cb(batteryStats);
  });
};

let getDate = (cb) => {
  runCmd('date "+%I:%M:%S%%%p"', (out) => cb(out.split('%').map((e) => e.replace('\n', ''))));
};

// A little hacky thing to use in calls to `aggregate()`
// Returns a function that will set `key` of the accumulator to the
// corresponding function's return value
let setKey = (key) => (obj, val) => obj[key] = val;

let getSystemInfo = (cb) => {
  aggregate({}, [
    [getDate,           setKey('date')],
    [getCPUStats,       setKey('cpu')],
    [getMemoryStats,    setKey('memory')],
    [getBatteryStats,   setKey('battery')],
    [getVolume,         setKey('volume')],
    [getWorkspaceLabel, setKey('workspace')]
  ], (stats) => cb(stats));
}

let progressBar = (val, max, width, text) => {
  let amt = Math.floor((val / max) * (width + 2));

  let out = ` ${text} ` + '~'.repeat(Math.max(width - text.length, 0));
  let fin = `${out.slice(0, amt)}%{R}${out.slice(amt)}`

  return `%{R}${fin}`;
}

let getMessage = (cb) => {
  const VOLUME_STEP = 3;

  const sep = `${setColor(theme.YELLOW)}%{T2} :: %{T1}${setColor(theme.GREEN)}`;

  getSystemInfo((stats) => {
    let charging = (stats.battery.POWER_SUPPLY_STATUS == 'Charging') ? `` : (stats.battery.POWER_SUPPLY_STATUS == 'Discharging') ? `` : ``;

    let volumeControls = [
      `%{A4:amixer -q sset Master ${VOLUME_STEP}%+:}`,
      `%{A5:amixer -q sset Master ${VOLUME_STEP}%-:}`,
      `%{A3:amixer -q sset Master toggle:}`,
      `%{A2:pavucontrol:}`
    ];
    let volumeColor = stats.volume.muted ? theme.WHITE : theme.GREEN;

    let volume = `${volumeControls.join('')} ${setColor(volumeColor)}${stats.volume.right}% ${'%{A}'.repeat(volumeControls.length)}`;
    let battery =  `${setColor(theme.BR_GREEN)}%{R} ${charging} ${setColor(theme.GREEN)}%{R} ${stats.battery.POWER_SUPPLY_CAPACITY}`;
    let memory = `%{T2}${setColor(theme.BLUE)}${stats.memory.readable.mem.used}%{T1}`;
    let cpu = `%{T2}${setColor(theme.BLUE)}${Math.round((os.loadavg()[0] / os.cpus().length) * 100)}% CPU%{T1}`;
    let date = `${setColor(theme.GREEN)}%{R}  ${stats.date[0]} %{T2}${stats.date[1]}%{T1}  %{R}`;
    let workspace = `${setColor(theme.GREEN)}%{R} ${stats.workspace} %{R}`;

    let msg = `%{l}${battery}% ${sep}${volume}%{c}${date}%{r}${memory}${sep}${cpu}${sep}${workspace}`;

    cb(msg);
  });
};

getMessage(console.log);

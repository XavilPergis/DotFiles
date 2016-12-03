const exec = require('child_process').exec;
const os = require('os');

// Load extensions of builtin objects
require('./builtins');

const BG_COLOR = '#002b36';
const FG_COLOR = '#859900';

const WHITE   = '#839496';
const BLACK   = '#002b36';
const RED     = '#dc322f';
const GREEN   = '#859900';
const BR_GREEN= '#9baf00';
const YELLOW  = '#b58900';
const BLUE    = '#268bd2';
const MAGENTA = '#d33682';
const CYAN    = '#2aa198';

const blackCode = `%{F${BLACK}}%{B${BLACK}}`;
const redCode = `%{F${RED}}%{B${BLACK}}`;
const greenCode = `%{F${GREEN}}%{B${BLACK}}`;
const yellowCode = `%{F${YELLOW}}%{B${BLACK}}`;
const blueCode = `%{F${BLUE}}%{B${BLACK}}`;
const magentaCode = `%{F${MAGENTA}}%{B${BLACK}}`;
const cyanCode = `%{F${CYAN}}%{B${BLACK}}`;

const setColor = (color) => `%{F${color}}%{B${BLACK}}`;

const aggregate = (accumulator, callbackList, aggregateCallback) => {
  let out = {};

  let runNext = (callbackUnit) => {
    let fn = callbackUnit[0];
    let callback = callbackUnit[1];

    fn((...args) => {
      callback(accumulator, ...args);
      let next = callbackList.pop()
      if(next) runNext(next);
      else aggregateCallback(accumulator);
    })
  }

  runNext(callbackList.pop());

};

// Object.defineProperty(Array.prototype, '', {
//   __proto__: null,
//   value: function() {
//
//   }
// });

let runCmd = (cmd, cb) => {
  exec(cmd, (err, stdout, stderr) => {
    if(!err) cb(stdout, stderr);
  });
};

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

// let getCPUTemperatures = (cb) => {
//   ;
// };

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

      return times;
    });

    runCmd("mpstat -P ALL | grep -P '\\d+:\\d+:\\d+\\s+(?:AM|PM)\\s+(?:all|\\d+)'", (out) => {
      let util = {};

      out.split('\n').trimLast().forEach((line) => {
        let stats = line.split(/\s+/).slice(2);
        let categories = ['user', 'nice', 'sys', 'iowait', 'irq', 'soft', 'steal', 'guest', 'gnice', 'idle'];

        let cpuName = stats[0] == 'all' ? 'average' : `cpu${stats[0]}`;

        util[cpuName] = categories.zipObject(stats.trimFirst());
      });

      icb(util);

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
    let memStats = {};
    out.split('\n').trimFirst().trimLast().forEach((line) => {
      let name = line.split(':')[0];
      let categories = ['total', 'used', 'free', 'shared', 'cache', 'available'];

      memStats[name.toLowerCase()] = categories.zipObject(line.split(':')[1].split(/\s+/).trimFirst());
    });

    cb(memStats);
  });
};

let humanReadableMem = (mem) => {
  let megs = Math.round(10 * mem / 1024) / 10; // Megs
  let gigs = Math.round(10 * mem / 1048576) / 10; // Gigs

  return gigs >= 1 ? gigs + ' GiB' : megs + ' MiB';
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


let setKey = (key) => {
  return (obj, val) => obj[key] = val;
};

let getSystemInfo = (cb) => {
  aggregate({}, [
    [getDate,           setKey('date')],
    [getCPUStats,       setKey('cpu')],
    [getMemoryStats,    setKey('memory')],
    [getBatteryStats,   setKey('battery')],
    [getVolume,         setKey('volume')],
    [getWorkspaceLabel, setKey('workspace')]
  ], (stats) => {
    cb(stats);
  });
}

let progressBar = (val, max, width, text) => {
  let amt = Math.floor((val / max) * (width + 2));

  let out = ` ${text} ` + '~'.repeat(Math.max(width - text.length, 0));
  let fin = `${out.slice(0, amt)}%{R}${out.slice(amt)}`

  return `%{R}${fin}`;
}

let getMessage = (cb) => {
  const VOLUME_STEP = 3;

  const sep = `${setColor(YELLOW)}%{T2} :: %{T1}${setColor(GREEN)}`;

  getSystemInfo((stats) => {
    let charging = (stats.battery.POWER_SUPPLY_STATUS == 'Charging') ? `` : (stats.battery.POWER_SUPPLY_STATUS == 'Discharging') ? `` : ``;

    let volumeControls = [
      `%{A4:amixer -q sset Master ${VOLUME_STEP}%+:}`,
      `%{A5:amixer -q sset Master ${VOLUME_STEP}%-:}`,
      `%{A3:amixer -q sset Master toggle:}`,
      `%{A2:pavucontrol:}`
    ];

    let volumeColor = stats.volume.muted ? WHITE : GREEN;

    let volume = `${volumeControls.join('')} ${setColor(volumeColor)}${stats.volume.right}% ${'%{A}'.repeat(volumeControls.length)}`;
    let battery =  `${setColor(BR_GREEN)}%{R} ${charging} ${setColor(GREEN)}%{R} ${stats.battery.POWER_SUPPLY_CAPACITY}`;
    let memory = `%{T2}${setColor(BLUE)}${humanReadableMem(stats.memory.mem.used)}%{T1}`;
    let cpu = `%{T2}${setColor(BLUE)}${Math.round((os.loadavg()[0] / os.cpus().length) * 100)}% CPU%{T1}`;
    let date = `${greenCode}%{R}  ${stats.date[0]} %{T2}${stats.date[1]}%{T1}  %{R}`;
    let workspace = `${setColor(GREEN)}%{R} ${stats.workspace} %{R}`;

    let msg = `%{l}${battery}% ${sep}${volume}%{c}${date}%{r}${memory}${sep}${cpu}${sep}${workspace}`;

    cb(msg);
  });
};

getMessage(console.log);

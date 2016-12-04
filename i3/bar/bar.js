const exec = require('child_process').exec;
const spawn = require('child_process').spawn;

const SCRIPT_TIMEOUT = 1000;
const FONT_SIZE = 10;
const BAR_PADDING = 7;

// Grabbed these from `fc-list`
const NORMAL_FONT = `RobotoMono Nerd Font:style=Regular:size=${FONT_SIZE}`;
const BOLD_FONT = `RobotoMono Nerd Font:style=Bold:size=${FONT_SIZE}`;

let lemonbar = spawn('lemonbar', ['-p', '-f', NORMAL_FONT, '-f', BOLD_FONT, '-u', BAR_PADDING]);

// Run the actual bar script. Redirect its stdout to the bar and its stderr to
// our process's stdout. Mostly used for debugging.
let runScript = () => {
  let script = spawn('node', [`${process.env.HOME}/.config/i3/bar/barscript.js`]);

  script.stdout.on('data', (data) => lemonbar.stdin.write(data));
  script.stderr.on('data', (data) => process.stdout.write(data));
};


// Runs whatever lemonbar outputs. Also runs the script again to facilitate
// instant feedback for things like the volume indicator.
lemonbar.stdout.on('data', (data) => {
  exec(data.toString(), () => {});
  runScript();
});

// Run the script every `SCRIPT_TIMEOUT` ms.
setInterval(() => {
  runScript();
}, SCRIPT_TIMEOUT);

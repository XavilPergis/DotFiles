const exec = require('child_process').exec;
const spawn = require('child_process').spawn;

let runScript = () => {
  let script = spawn('node', [`${process.env.HOME}/.config/i3/barscript.js`]);

  script.stdout.on('data', (data) => {
    lemonbar.stdin.write(data);
  });

  script.stderr.on('data', (data) => {
    process.stdout.write(data);
  });
};

const FONT_SIZE = 10;

const NORMAL_FONT = `RobotoMono Nerd Font:style=Regular:size=${FONT_SIZE}`;
const BOLD_FONT = `RobotoMono Nerd Font:style=Bold:size=${FONT_SIZE}`;
const BAR_PADDING = '7';

let lemonbar = spawn('lemonbar', ['-p', '-f', NORMAL_FONT, '-f', BOLD_FONT, '-u', BAR_PADDING]);

lemonbar.stdout.on('data', (data) => {
  exec(data.toString(), () => {});
  runScript();
});

setInterval(() => {
  runScript();
}, 500);

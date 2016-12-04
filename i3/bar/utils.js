const readableMem = (val) => {
  let megs = Math.round(10 * val / 1024) / 10; // Megs
  let gigs = Math.round(10 * val / 1048576) / 10; // Gigs

  return gigs >= 1 ? gigs + ' GiB' : megs + ' MiB';
};

const theme = {
  WHITE:    '#839496',
  BLACK:    '#002b36',
  RED:      '#dc322f',
  GREEN:    '#859900',
  YELLOW:   '#b58900',
  BLUE:     '#268bd2',
  MAGENTA:  '#d33682',
  CYAN:     '#2aa198',

  BR_GREEN: '#9baf00'
};


module.exports = {
  readableMem: readableMem,
  theme: theme
};

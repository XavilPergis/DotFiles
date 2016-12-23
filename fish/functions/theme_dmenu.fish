function __dmenu_run
  set width 600
  set height 30

  set xoff (math (screendim)[1] / 2 - $width / 2)
  set yoff (math (screendim)[2] / 2 - $height / 2)

  dmenu -i -x $xoff -y $yoff -h $height -w $width -nb '#073642' -sf '#073642' -sb '#859900' $argv
end

function theme_dmenu -d ''
  __dmenu_run $argv
end

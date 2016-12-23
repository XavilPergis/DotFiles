function __dmenu_run
  set width 600
  set height 30

  set xoff (math (screendim)[1] / 2 - $width / 2)
  set yoff (math (screendim)[2] / 2 - $height / 2)

  dmenu -i -p 'Run Command:' -x $xoff -y $yoff -h $height -w $width -nb '#073642' -sf '#073642' -sb '#859900'
end

function launcher -d 'A command launcher using dmenu'
  set stdout /tmp/LAUNCHER_STDOUT
  set stderr /tmp/LAUNCHER_STDERR

  for execdir in $PATH
    find $execdir -executable -type f -or -type l | grep -oP '/\K\w+$'
  end | sort -u | __dmenu_run | begin

    for f in $stdout $stderr
      test ! -e $f; and mkfifo $f
    end

    read cmd
    eval $cmd
  end
end

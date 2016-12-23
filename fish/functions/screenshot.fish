function screenshot -d "Take a screenshot"
  # Some basic config :p
  set out_file '/tmp/screenshot.png'
  set color '0.6901,0.7803,0,0.2509'

  # Take the screenshot
  if test $argv[1]; and test $argv[1] = '--select'
    maim -s -l -c $color -- $out_file
  else
    maim -l -c $color -- $out_file
  end

  feh -g 640x480 -Z $out_file &

  # Copy to clipboard
  xclip -sel clip -t (file -b --mime-type $out_file) < $out_file
end

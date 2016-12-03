function screendim -d 'Find the dimensions of the screen.'
  set dimstr (xdpyinfo | grep -oP 'dimensions:\s+\K\d+x\d+')

  set width (echo -n $dimstr | grep -oP '^\d+')
  set height (echo -n $dimstr | grep -oP '\d+$')

  echo $width\n$height
end

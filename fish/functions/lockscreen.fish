function lockscreen -d "Lock the screen with a modified i3-lock"
  set background_color      (theme_color black)
  set inner_color_verifying (theme_color bright_black ff)
  set inner_color_wrong     (theme_color bright_black ff)
  set inner_color           (theme_color bright_black ff)
  set ring_color_verifying  (theme_color green ff)
  set ring_color_wrong      (theme_color magenta ff)
  set ring_color            (theme_color bright_black ff)
  set line_color            (theme_color green ff)
  set separator_color       (theme_color green ff)
  set text_color            (theme_color blue ff)
  set keypress_highlight    (theme_color green ff)
  set backspace_highlight   (theme_color yellow ff)

  set time_string "%I:%M:%S"
  set date_string "%a, %b %m, %Y"

  i3lock -k --color=$background_color --insidevercolor=$inner_color_verifying --insidewrongcolor=$inner_color_wrong --insidecolor=$inner_color --ringvercolor=$ring_color_verifying --ringwrongcolor=$ring_color_wrong --ringcolor=$ring_color --linecolor=$line_color --separatorcolor=$separator_color --textcolor=$text_color --keyhlcolor=$keypress_highlight --bshlcolor=$backspace_highlight --timestr=$time_string --datestr=$date_string
end

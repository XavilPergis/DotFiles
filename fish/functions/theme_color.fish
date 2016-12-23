function theme_color -d "Gets theme colors."

  # Color definitions
  set white        "839496"
  set black        "002b36"
  set red          "dc322f"
  set green        "859900"
  set yellow       "b58900"
  set blue         "268bd2"
  set magenta      "d33682"
  set cyan         "2aa198"
  set bright_black "073642"
  set bright_green "9baf00"

  if test (count $argv) = "2"
    set alpha $argv[2]
  end

  if test (count $argv) = "0"
    echo "white=$white"
    echo "black=$black"
    echo "red=$red"
    echo "green=$green"
    echo "yellow=$yellow"
    echo "blue=$blue"
    echo "magenta=$magenta"
    echo "cyan=$cyan"
    echo "bright_black=$bright_black"
    echo "bright_green=$bright_green"
  else
    switch $argv[1]
      case "white";         echo (string join "" $white $alpha)
      case "black";         echo (string join "" $black $alpha)
      case "red";           echo (string join "" $red $alpha)
      case "green";         echo (string join "" $green $alpha)
      case "yellow";        echo (string join "" $yellow $alpha)
      case "blue";          echo (string join "" $blue $alpha)
      case "magenta";       echo (string join "" $magenta $alpha)
      case "cyan";          echo (string join "" $cyan $alpha)
      case "bright_black";  echo (string join "" $bright_black $alpha)
      case "bright_green";  echo (string join "" $bright_green $alpha)
    end
  end

end

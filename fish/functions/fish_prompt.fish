function fish_prompt

	# echo $status

	set exit_status $status

	if test $exit_status -eq 0
		set exit_color green
	else
		set exit_color red
	end

	if test (whoami) = 'root'
		set user_prompt '#'
		set user_color red
	else
		set user_prompt (set_color -o)'$'(set_color normal)
		set user_color yellow
	end

	# Are we on a TTY or a fancier terminal?
  tty | string match -q -r tty; and set tty tty; or set tty pts

	set exit_fmt (set_color -o $exit_color)$exit_status(set_color normal)
	set user (set_color -o $user_color)$USER(set_color normal)
	set host (set_color -o blue)(hostname)(set_color normal)
	set date_fmt '['(set_color magenta)(date "+%X")(set_color normal)']'
	set cwd (pwd | sed "s=$HOME=~=")

	printf '(%s) %s@%s:%s %s' $exit_fmt $user $host $cwd $date_fmt
	printf '\n%s ' $user_prompt

end

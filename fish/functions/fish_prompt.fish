# Fish git prompt
set __fish_git_prompt_showdirtystate 'yes'
set __fish_git_prompt_showstashstate 'yes'
set __fish_git_prompt_showuntrackedfiles 'yes'
set __fish_git_prompt_showupstream 'yes'
set __fish_git_prompt_color_branch yellow
set __fish_git_prompt_color_upstream_ahead green
set __fish_git_prompt_color_upstream_behind red

# Status Chars
set __fish_git_prompt_char_dirtystate '⚡'
set __fish_git_prompt_char_stagedstate '→'
set __fish_git_prompt_char_untrackedfiles (set_color -o red)"!"(set_color normal)
set __fish_git_prompt_char_stashstate '↩'
set __fish_git_prompt_char_upstream_ahead '+'
set __fish_git_prompt_char_upstream_behind '-'

set GIT_DIRTY_TEXT '!'

function git_prompt
	set in_git_repo (git status 2> /dev/null | grep "On branch" | wc -l)

	if test $in_git_repo = 1
		set dirty_info_raw (git diff --shortstat | grep -oP '\d+')

		if test $dirty_info_raw[1]
			set git_dirty $GIT_DIRTY_TEXT
			# set git_additions $dirty_info_raw[2]
			# set git_deletions $dirty_info_raw[3]
		end

		set gitp (printf $argv[1] '' $git_dirty)
	end
end

function base_prompt
	if test $PREVIOUS_EXIT_STATUS -eq 0
		set exit_color green
	else
		set exit_color red
	end

	# Are we on a TTY or a fancier terminal?
  tty | string match -q -r tty; and set tty tty; or set tty pts

	set exit_fmt (set_color -o $exit_color)$PREVIOUS_EXIT_STATUS(set_color normal)
	set user (set_color -o $user_color)$USER(set_color normal)
	set host (set_color -o blue)(hostname)(set_color normal)
	set date_fmt '['(set_color magenta)(date "+%_I:%M")(set_color normal)']'
	set cwd (pwd | sed "s=$HOME=~=")

	printf $argv[1] $exit_fmt $user $host $cwd $date_fmt
end

function end_string
	if test (whoami) = 'root'
		set user_prompt '#'
		set user_color red
	else
		set user_prompt (set_color -o)'$'(set_color normal)
		set user_color yellow
	end
end

function fish_prompt
	set -U PREVIOUS_EXIT_STATUS $status

	set base_prompt_fmt '(%s) %s@%s: %s %s%s'
	set git_prompt_fmt '%s%s'

	if test (whoami) = 'root'
		set user_prompt '#'
		set user_color red
	else
		set user_prompt (set_color -o)'$'(set_color normal)
		set user_color yellow
	end

	printf '%s%s' (base_prompt $base_prompt_fmt) (git_prompt $git_prompt_fmt)
	printf '\n%s ' (set_color -o $user_color)$user_prompt(set_color normal)
end

function songlist -d "Lists songs in mpc playlist"
  set pl (mpc playlist)

  set track (mpc playlist | theme_dmenu -l 10)

  if test $track
    for i in (seq 1 (count $pl))
      if test $pl[$i] = $track
        set number $i
      end
    end

    mpc play $number
  end
end

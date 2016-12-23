#![feature(box_syntax)]
#![feature(box_patterns)]

extern crate mpd;
extern crate regex;
extern crate sys_info;
extern crate time;

mod format;
mod stats;
mod bar;
mod pair;

use format::*;
use stats::battery::*;
use stats::stats::Stats;
use stats::volume::*;
use bar::{ Lemonbar, BarPosition };

use std::thread;

use mpd::Client;
use mpd::status::State as MpdState;

fn progress_bar(val: u64, max: u64, width: u64) -> String {
    let amt = (width * val) / max;

    let head = (0..amt).fold(String::new(), |b, _| b + "");
    let tail = (0..(width - amt)).fold(String::new(), |b, _| b + " ");

    format!("{}{}", head, tail)
}

fn main() {

    // Because stuff gets extroardinarily verbose.
    use Segment::*;
    use TextFormat::*;
    use ThemeColor::*;

    let mut mpd_bar = Lemonbar::new(BarPosition::Bottom);

    mpd_bar.add_segment("Song Name", box move |tx| {
        // TODO: This gets called *a lot*
        let mut conn = Client::connect("127.0.0.1:6600").unwrap();
        let song = conn.currentsong().unwrap();

        let name = Formatted(
            ColorPair(Blue, Black),
            box Text(format!(" {} ", match song {
                Some(s) => s.name.unwrap_or("<Unknown>".into()),
                None => "No Song".into()
            }).into())
        );

        tx.send(name).unwrap();
    });

    mpd_bar.add_segment("Player Controls", box |tx| {
        // TODO: This gets called *a lot*
        let mut conn = Client::connect("127.0.0.1:6600").unwrap();

        let icon = match conn.status().unwrap().state {
            MpdState::Stop => "",
            MpdState::Play => "",
            MpdState::Pause => ""
        };

        let segment = Compound(vec![
            Formatted(
                ColorPair(Black, Green),
                box Action(MouseEvent::LeftClick, "mpc prev".into(), box Text(format!("  ")))
            ),
            Formatted(
                ColorPair(Black, BrightGreen),
                box Action(MouseEvent::LeftClick, "mpc toggle".into(), box Text(format!(" {} ", icon)))
            ),
            Formatted(
                ColorPair(Black, Green),
                box Action(MouseEvent::LeftClick, "mpc next".into(), box Text(format!("  ")))
            ),
        ]);

        tx.send(segment).unwrap();
    });

    mpd_bar.add_segment("Progress Bar", box |tx| {
        // TODO: This gets called *a lot*
        let mut conn = Client::connect("127.0.0.1:6600").unwrap();
        let status = conn.status().unwrap();

        // Cast to `u64` because out durations can't be negative. ("My song is -5 minutes long!")
        let elapsed_seconds = status.elapsed.map(|e| (e.num_seconds() / 1000) as u64);
        let elapsed_minutes = status.elapsed.map(|e| (e.num_minutes() / 1000) as u64);
        let length_seconds = status.time.map(|e| e.1.num_seconds() as u64);

        let timer = match elapsed_seconds {
            Some(_) => format!(" {0}:{1:0>2} ", elapsed_minutes.unwrap_or(0) % 60, elapsed_seconds.unwrap_or(0) % 60),
            None => " Stopped ".into()
        };

        let segment = Compound(vec![
            Formatted(
                Multi(vec![Right, ColorPair(Blue, Black)]),
                box Text(timer)
            ),
            Formatted(
                ColorPair(Green, Black),
                box Text(format!(" {}  ", progress_bar(elapsed_seconds.unwrap_or(0), length_seconds.unwrap_or(1), 50)))
            )
        ]);

        tx.send(segment).unwrap();
    });

    // Box syntax because oh my god.
    let mut main_bar = Lemonbar::new(BarPosition::Top);

    main_bar.add_segment("Battery", box |tx| {
        use BatteryChargeState::*;
        let charging_icon = match get_battery_info().state {
            Full        => "".into(),
            Charging    => "".into(),
            Discharging => "".into(),
            Unknown(s)  => s
        };

        let segment = Compound(vec![
            Formatted(
                ColorPair(Black, BrightGreen),
                box Text(format!(" {} ", String::from(charging_icon)))
            ),
            Formatted(
                ColorPair(Black, Green),
                box Text(format!(" {}% ", get_battery_info().charge.unwrap_or(0)))
            )
        ]);

        tx.send(segment).unwrap();
    });

    main_bar.add_segment("Volume Control", box |tx| {
        let vol_color = if get_volume().right.muted.unwrap() { Green } else { White };
        tx.send(Formatted(
            ColorPair(vol_color, Black),
            box Action(MouseEvent::ScrollUp, String::from("amixer sset Master 4%+"),
                box Action(MouseEvent::ScrollDown, String::from("amixer sset Master 4%-"),
                    box Action(MouseEvent::RightClick, String::from("amixer sset Master toggle"),
                        box Text(format!(" {}% ", get_volume().get_string())))))
        )).unwrap();
    });

    main_bar.add_segment("Time", box |tx| {
        tx.send(Formatted(
            Multi(vec![Center, ColorPair(Green, BrightBlack)]),
            box Text(format!(" {} ", time::now().strftime("%I:%M:%S %p").unwrap()))
        )).unwrap();
    });

    // Set up segment ordering
    main_bar.set_order(vec!["Battery", "Volume Control", "Time"]);
    mpd_bar.set_order(vec!["Player Controls", "Song Name", "Progress Bar"]);

    // Get string, Write string. Rinse and repeat.
    loop {
        let main_bar_out = main_bar.get_bar_string();
        main_bar.write(main_bar_out).unwrap();

        let mpd_bar_out = mpd_bar.get_bar_string();
        mpd_bar.write(mpd_bar_out).unwrap();

        // Sleep so we don't kill our CPU
        thread::sleep(std::time::Duration::from_millis(250));
    }

}

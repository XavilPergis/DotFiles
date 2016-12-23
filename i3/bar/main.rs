#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(unmarked_api)]
#![feature(io)]

extern crate mpd;
extern crate sys_info;
extern crate time;

use std::thread;
use std::io::prelude::*;
use std::io;
use std::sync::{ Arc, Mutex };
use std::sync::mpsc::channel;
use std::error::Error;
use std::net::TcpStream;
use std::collections::HashMap;
use std::process::{ Command, Stdio, Child, ChildStdin, ChildStdout };

use mpd::Client;

const WHITE: &'static str    = "839496";
const BLACK: &'static str    = "002b36";
const RED: &'static str      = "dc322f";
const GREEN: &'static str    = "859900";
const YELLOW: &'static str   = "b58900";
const BLUE: &'static str     = "268bd2";
const MAGENTA: &'static str  = "d33682";
const CYAN: &'static str     = "2aa198";

const BR_BLACK: &'static str = "073642";
const BR_GREEN: &'static str = "9baf00";

#[derive(Clone, Debug)]
enum TextFormat {
    Foreground(String),
    Background(String),
    Underline(String),
    Font(usize),
    Swap,
    Left,
    Right,
    Center,
    Multi(Box<TextFormat>, Box<TextFormat>)
}

#[derive(Clone, Debug)]
enum MouseEvent {
    LeftClick,
    MiddleClick,
    RightClick,
    ScrollUp,
    ScrollDown
}

#[derive(Clone, Debug)]
enum Segment {
    Compound(Box<Segment>, Box<Segment>),
    Formatted(TextFormat, Box<Segment>),
    Action(MouseEvent, String, Box<Segment>),
    Text(String)
}

unsafe impl Send for Segment {}

struct Lemonbar {
    segments: Vec<Box<Fn() -> Segment>>,
    process: Arc<Child>
}

impl Lemonbar {
    fn new() -> Lemonbar {
        let process = match Command::new("lemonbar")
                                    .args(&["-p", "-u", "7", "-f", "RobotoMono Nerd Font:style=Regular:size=10"])
                                    .stdin(Stdio::piped())
                                    .stdout(Stdio::piped())
                                    .spawn() {
            Err(why) => panic!("Failed to run lemonbar. {}", why.description()),
            Ok(process) => process
        };

        Lemonbar {
            segments: Vec::new(),
            process: Arc::new(process)
        }
    }

    fn add_segment<F>(mut self, update_fn: Box<F>) -> Lemonbar whercd e F: Send + Sync + 'static + Fn() -> Segment {
        self.segments.push(update_fn);
        self
    }

    fn run(&mut self) {
        thread::Builder::new().name(String::from("Bar Input")).spawn(move || {
            loop {
                thread::sleep(std::time::Duration::from_millis(250));

                let mut string_builder = String::new();

                for segment_fn in self.arg.iter() {
                    string_builder += process_segment(segment_fn()).as_str();
                }

                string_builder += "%{B#073642}\n";

                self.process.stdin.as_mut().unwrap().write(string_builder.as_bytes());
            }
        });
        thread::Builder::new().name(String::from("Bar Output")).spawn(move || {
            loop {
                thread::sleep(std::time::Duration::from_millis(250));

            }
        });

        loop {

        }

        // let mut out_buf = Vec::new();
        //
        // self.process.stdout.read_to_end(&mut out_buf);
        //
        // io::stdout().write(out_buf.into());
    }
}

fn process_segment(seg: Segment) -> String {
    use Segment::*;
    use MouseEvent::*;

    match seg {
        Compound(box first_child, box second_child) => {
            format!("{}{}", process_segment(first_child), process_segment(second_child))
        },
        Formatted(fmt, box child_segment) => {
            format!("{}{}", process_format(fmt), process_segment(child_segment))
        },
        Action(evt, cmd, box child_segment) => {
            let key = match evt {
                LeftClick => 1,
                MiddleClick => 2,
                RightClick => 3,
                ScrollUp => 4,
                ScrollDown => 5
            };

            format!("%{{A{}:{}:}}{}%{{A}}", key, cmd, process_segment(child_segment))
        },
        Text(string) => string
    }
}

fn process_format(fmt: TextFormat) -> String {
    use TextFormat::*;
    String::from(match fmt {
        Foreground(color) => format!("%{{F#{}}}", color),
        Background(color) => format!("%{{B#{}}}", color),
        Underline(color)  => format!("%{{U#{}}}", color),
        Font(index)       => format!("%{{T#{}}}", index),
        Swap              => String::from("%{R}"),
        Left              => String::from("%{l}"),
        Right             => String::from("%{r}"),
        Center            => String::from("%{c}"),
        Multi(box first_child, box second_child) => format!("{}{}", process_format(first_child), process_format(second_child))
    })
}

enum BatteryChargeState {
    Full,
    Charging,
    Discharging,
    Unknown(String)
}

struct BatteryInfo {
    charge: Option<i64>,
    state: BatteryChargeState
}

fn get_battery_info() -> BatteryInfo {
    let output = Command::new("cat").arg("/sys/class/power_supply/BAT1/uevent").stdout(Stdio::piped()).output().unwrap();
    let stats = String::from_utf8_lossy(&output.stdout);

    let mut batteryMap: HashMap<&str, &str> = HashMap::new();

    for line in stats.split('\n') {
        let kv: Vec<&str> = line.split('=').collect();

        if kv.len() == 2 {
            batteryMap.insert(kv[0], kv[1]);
        }
    }

    let batteryState = match batteryMap.get("POWER_SUPPLY_STATUS") {
        Some(val) => {
            match *val {
                "Full" => BatteryChargeState::Full,
                "Discharging" => BatteryChargeState::Discharging,
                "Charging" => BatteryChargeState::Charging,
                other => BatteryChargeState::Unknown(other.into())
            }
        },
        None => BatteryChargeState::Unknown("?".into())
    };

    BatteryInfo {
        charge: batteryMap.get("POWER_SUPPLY_CAPACITY").map(|val| val.parse::<i64>().unwrap_or(-1)),
        state: batteryState
    }
}

fn get_volume() -> String {
    let output = Command::new("fish")
                        .arg("-c")
                        .arg("amixer -c 1 -D pulse get Master | grep -Po \"\\d+%\"")
                        .stdout(Stdio::piped())
                        .output().unwrap();

    let stats = String::from_utf8_lossy(&output.stdout);
    let volumes: Vec<&str> = stats.split('\n').collect();
    volumes[0].to_string()
}

fn main() {
    let mut mpc_connection = Client::connect("127.0.0.1:6600").unwrap();

    use Segment::*;
    use TextFormat::*;

    let mut bar = Lemonbar::new()
    .add_segment(box || {
        use BatteryChargeState::*;
        let charging_icon = match get_battery_info().state {
            Full        => "".into(),
            Charging    => "".into(),
            Discharging => "".into(),
            Unknown(s)  => s
        };

        Compound(
            box Formatted(
                Multi(box Background(BR_GREEN.into()), box Foreground(BLACK.into())),
                box Text(format!(" {} ", String::from(charging_icon)))
            ), box Formatted(
                Multi(box Background(GREEN.into()), box Foreground(BLACK.into())),
                box Text(format!(" {}% ", get_battery_info().charge.unwrap_or(0)))
            )
        )
    })
    .add_segment(box || {
        Formatted(
            Multi(box Foreground(GREEN.into()), box Background(BR_BLACK.into())),
            box Action(MouseEvent::ScrollUp, String::from("amixer sset Master 15%+"),
                box Action(MouseEvent::ScrollDown, String::from("amixer sset Master 15%-"),
                    box Action(MouseEvent::RightClick, String::from("amixer sset Master toggle"),
                        box Text(format!(" {} ", get_volume())))))
        )
    })
    .add_segment(box || {
        Formatted(
            Multi(box Center, box Multi(box Foreground(GREEN.into()), box Background(BR_BLACK.into()))),
            box Text(format!(" {} ", time::now().strftime("%I:%M:%S").unwrap()))
        )
    });

    bar.run();

}

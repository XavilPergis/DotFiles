use std::process::{ Command, Stdio };

use regex::Regex;

use stats::stats::Stats;

#[derive(Copy, Clone, Debug)]
pub struct VolumeStats {
    pub left: ChannelStats,
    pub right: ChannelStats
}

#[derive(Copy, Clone, Debug)]
pub struct ChannelStats {
    pub absolute: Option<u64>,
    pub percent: Option<u64>,
    pub muted: Option<bool>
}

pub fn get_volume() -> VolumeStats {
    let output = Command::new("fish")
                        .arg("-c")
                        .arg("amixer -c 1 -D pulse get Master")
                        .stdout(Stdio::piped())
                        .output().unwrap();

    let re = Regex::new(r"Front.*:\s+Playback\s+(?P<raw>\d+)\s+\[(?P<percent>\d+)%\]\s*\[(?P<muted>on|off)\]").unwrap();

    let out_str = String::from_utf8_lossy(&output.stdout).into_owned();
    let mut chans: Vec<ChannelStats> = Vec::new();

    for cap in re.captures_iter(out_str.as_str()) {
        chans.push(ChannelStats {
            absolute: cap.name("raw").and_then(|e| e.parse::<u64>().ok()),
            percent: cap.name("percent").and_then(|e| e.parse::<u64>().ok()),
            muted: cap.name("muted").and_then(|e| match e {
                "on" => Some(true),
                "off" => Some(false),
                _ => None
            })
        });
    }

    VolumeStats {
        left: chans[0],
        right: chans[1]
    }
}

impl Stats for VolumeStats {
    fn get_string(&self) -> String {
        format!("{}", self.right.percent.unwrap())
    }
}

use std::process::{ Command, Stdio };
use std::collections::HashMap;

pub enum BatteryChargeState {
    Full,
    Charging,
    Discharging,
    Unknown(String)
}

pub struct BatteryInfo {
    pub charge: Option<i64>,
    pub state: BatteryChargeState
}

pub fn get_battery_info() -> BatteryInfo {
    let output = Command::new("cat").arg("/sys/class/power_supply/BAT1/uevent").stdout(Stdio::piped()).output().unwrap();
    let stats = String::from_utf8_lossy(&output.stdout);

    let mut mattery_map: HashMap<&str, &str> = HashMap::new();

    for line in stats.split('\n') {
        let kv: Vec<&str> = line.split('=').collect();

        if kv.len() == 2 {
            mattery_map.insert(kv[0], kv[1]);
        }
    }

    let battery_state = match mattery_map.get("POWER_SUPPLY_STATUS") {
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
        charge: mattery_map.get("POWER_SUPPLY_CAPACITY").map(|val| val.parse::<i64>().unwrap_or(-1)),
        state: battery_state
    }
}

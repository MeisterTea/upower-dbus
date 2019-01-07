extern crate upower_dbus;

use std::process::exit;
use upower_dbus::UPower;

fn main() {
    let upower = match UPower::new(1000) {
        Ok(upower) => upower,
        Err(why) => {
            eprintln!("failed to get dbus connection: {}", why);
            exit(1);
        }
    };

    let print_state = |state| println!("{}", &format!("battery state: {}", state));

    match upower.get_state() {
        Ok(0) => print_state("unknown"),
        Ok(1) => print_state("charging"),
        Ok(2) => print_state("discharging"),
        Ok(3) => print_state("empty"),
        Ok(4) => print_state("fully charged"),
        Ok(5) => print_state("pending charge"),
        Ok(6) => print_state("pending discharge"),
        Ok(_) => {},
        Err(why) => {
            eprintln!("could not get battery state: {}", why);
            exit(1);
        }
    }
}

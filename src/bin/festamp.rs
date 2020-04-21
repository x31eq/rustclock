use num_integer::mod_floor;
use rustclock::Time;

fn main() {
    let instant = Time::from_args();
    print!(
        "{:03x}{:x}",
        mod_floor(instant.quarter, 0x1000),
        instant.week
    );
    println!(
        ".{:x}{:x}{:02x}",
        instant.halfday, instant.hour, instant.tick
    );
}

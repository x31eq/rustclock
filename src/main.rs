use num_integer::mod_floor;
use rustclock::Time;

fn main() {
    let now = Time::now();
    print!("{:03x}{:01x}", mod_floor(now.quarter, 0x1000), now.week);
    println!(".{:01x}{:01x}{:02x}", now.halfday, now.hour, now.ticks);
}

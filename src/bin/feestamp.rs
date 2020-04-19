use num_integer::mod_floor;
use rustclock::Time;

fn main() {
    let now = Time::now();
    print!("{:02x}{:x}", mod_floor(now.quarter, 0x100), now.week);
    println!("{:x}:{:x}{:02x}{:x}", now.halfday, now.hour, now.tick, now.sec);
}

use rustclock::Time;

fn main() {
    let instant = Time::from_args();
    println!(
        "{:02x}{:x}{:x}:{:x}{:02x}{:x}",
        instant.quarter & 0xff,
        instant.week,
        instant.halfday,
        instant.hour,
        instant.tick,
        instant.sec,
    );
}

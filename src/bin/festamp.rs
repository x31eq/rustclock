use rustclock::Time;

fn main() {
    let instant = Time::from_args();
    println!(
        "{:03x}{:x}.{:x}{:x}{:02x}",
        instant.quarter & 0xfff,
        instant.week,
        instant.halfday,
        instant.hour,
        instant.tick,
    );
}

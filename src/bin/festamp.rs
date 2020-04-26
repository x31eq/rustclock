use rustclock::Time;

fn main() {
    let instant = Time::from_args();
    print!("{:03x}{:x}", instant.quarter & 0xfff, instant.week);
    println!(
        ".{:x}{:x}{:02x}",
        instant.halfday, instant.hour, instant.tick,
    );
}

use rustclock::Time;

fn main() {
    let instant = Time::from_args();
    print!("{:02x}{:x}", instant.quarter & 0xff, instant.week);
    println!(
        "{:x}:{:x}{:02x}{:x}",
        instant.halfday, instant.hour, instant.tick, instant.sec,
    );
}

use std::thread;
use std::time::Duration;

fn simulated_expensive_calc(intensitiy: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensitiy
}
fn main() {
    println!("{}", simulated_expensive_calc(1));
}

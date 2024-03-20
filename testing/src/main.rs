use reverse_string;
use clock::Clock;

fn main() {

    let new_clock = Clock::new(8, 5);

    let clock1 = Clock::new(12, 45);

    let clock = Clock::new(5, 32).add_minutes(1500);

    // println!("{new_clock}");
    // println!("{clock1}");

    println!("{clock}");


}

mod parts;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    parts::part_1::part_1();
    parts::part_2::part_2();
    let elapsed: std::time::Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

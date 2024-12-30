mod parts;
use std::env;
use std::time::Instant;

fn main() {
    // Retrieve filename from command-line argument
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let now = Instant::now();

    parts::part_1::part_1(filename);
    parts::part_2::part_2(filename);

    let elapsed: std::time::Duration = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

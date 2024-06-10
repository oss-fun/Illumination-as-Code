use compute_pi::{compute_pi, init_thread_pool, parse_args};
use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let (num_threads, color) = parse_args(&args);

    init_thread_pool(num_threads);

    let num_rects: i64 = 2 * 1_000 * 1_000 * 1_000;
    let width: f64 = 1.0 / num_rects as f64;

    for _ in 0..10 {
        let area: f64 = compute_pi(num_rects, width);
        //println!("Computed Pi = {:.11}\n", area);
        print!("{}", color);
        io::stdout().flush().unwrap();
    }
}

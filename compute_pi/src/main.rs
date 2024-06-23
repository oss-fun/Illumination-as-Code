use compute_pi::{compute_pi, init_thread_pool, parse_args, send_chroma_base};
use std::env;
use std::io::{self, Write};

#[actix_web::main]
async fn main() -> Result<(), reqwest::Error> {
    // args
    let args: Vec<String> = env::args().collect();
    let (num_threads, color) = parse_args(&args);

    // rayon
    init_thread_pool(num_threads);

    // pi
    let num_rects: i64 = 1 * 1_000 * 1_000 * 100;
    let width: f64 = 1.0 / num_rects as f64;

    // test local server
    let url: &str = "http://192.168.22.5:8080";

    loop {
        let area: f64 = compute_pi(num_rects, width);
        println!("Computed Pi = {:.11}", area);
        io::stdout().flush().unwrap();

        if let Err(e) = send_chroma_base(url, color).await {
            println!("Error: {}", e);
        }
    }
}

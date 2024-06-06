use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::env;

pub fn parse_args() -> (usize, usize) {
    let args: Vec<String> = env::args().collect();
    let num_threads: usize = args[1].parse().expect("error");
    let color: usize = args[2].parse().expect("error");
    (num_threads, color)
}

pub fn init_thread_pool(num_threads: usize) {
    ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();
}

pub fn compute_pi(num_rects: i64, width: f64) -> f64 {
    let sum: f64 = (0..num_rects)
        .into_par_iter()
        .map(|i| {
            let mid = (i as f64 + 0.5) * width;
            let height = 4.0 / (1.0 + mid * mid);
            height
        })
        .sum();

    width * sum
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_args() {
        let args: Vec<String> = vec!["program".to_string(), "4".to_string(), "1".to_string()];
        env::set_var("CARGO_BIN_EXE_test_program", &args[0]);
        env::set_var("ARGS1", &args[1]);
        env::set_var("ARGS2", &args[2]);
        let parsed_args = parse_args();
        assert_eq!(parsed_args, (4, 1));
    }
    #[test]
    fn test_init_thread_pool() {
        init_thread_pool(4);
        assert!(true);
    }
}

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use reqwest::Client;

pub fn parse_args(args: &[String]) -> (usize, &str) {
    let num_threads: usize = args[1].parse().expect("error");
    let color: &str = &args[2];
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

pub async fn send_chroma_base(url: &str, color: &str) -> Result<(), reqwest::Error> {
    // サーバにリクエストを送信
    let client = Client::new();

    match client
        .post(url.to_string() + "/receive")
        .body(color.to_string())
        .send()
        .await
    {
        Ok(res) => {
            let status = res.status();
            let body = res.text().await.unwrap();
            println!("Status: [{}], Body: [{}]", status, body);
        }
        Err(err) => {
            println!("Request error: {}", err);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_args() {
        let args: Vec<String> = vec!["program".to_string(), "4".to_string(), "R".to_string()];
        let parsed_args = parse_args(&args);
        assert_eq!(parsed_args, (4, "R"));
    }
    #[test]
    fn test_init_thread_pool() {
        init_thread_pool(4);
        assert!(true);
    }
}

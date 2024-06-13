use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Chroma {
    red: u8,
    green: u8,
    blue: u8,
}

pub fn set_rnd_color_chroma3(rng: &mut impl Rng) -> (u8, u8, u8) {
    let mut color: (u8, u8, u8) = (0, 0, 0);
    let chromas = [0, 127, 255];

    color.0 = chromas[rng.gen_range(0..chromas.len())];
    color.1 = chromas[rng.gen_range(0..chromas.len())];
    color.2 = chromas[rng.gen_range(0..chromas.len())];
    color
}

pub fn set_rnd_color_chroma4(rng: &mut impl Rng) -> (u8, u8, u8) {
    let mut color: (u8, u8, u8) = (0, 0, 0);
    let chromas = [0, 85, 170, 255];

    color.0 = chromas[rng.gen_range(0..chromas.len())];
    color.1 = chromas[rng.gen_range(0..chromas.len())];
    color.2 = chromas[rng.gen_range(0..chromas.len())];
    color
}

pub async fn set_chroma_static(url: &str) -> Result<(u8, u8, u8), reqwest::Error> {
    let client = Client::new();

    match client
        .post(url.to_string() + "/get-chroma-static")
        .body("10")
        .send()
        .await
    {
        Ok(res) => {
            let chroma: Chroma = res.json().await.unwrap();
            Ok((chroma.red, chroma.green, chroma.blue))
        }
        Err(err) => {
            println!("Request error: {}", err);
            Err(err)
        }
    }
}

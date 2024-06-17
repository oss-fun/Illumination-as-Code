use rand::{thread_rng, Rng};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Chroma {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub fn set_rnd_color_chroma3() -> Chroma {
    let mut rng = thread_rng();
    let chromas = [0, 127, 255];
    let chroma = Chroma {
        red: chromas[rng.gen_range(0..chromas.len())],
        green: chromas[rng.gen_range(0..chromas.len())],
        blue: chromas[rng.gen_range(0..chromas.len())],
    };
    chroma
}

pub fn set_rnd_color_chroma4() -> Chroma {
    let mut rng = rand::thread_rng();
    let chromas = [0, 85, 170, 255];
    let mut chroma = Chroma {
        red: chromas[rng.gen_range(0..chromas.len())],
        green: chromas[rng.gen_range(0..chromas.len())],
        blue: chromas[rng.gen_range(0..chromas.len())],
    };

    if chroma.red == 0 && chroma.green == 0 && chroma.blue == 0 {
        chroma = set_rnd_color_chroma4();
    } else if chroma.red == 255 && chroma.green == 255 && chroma.blue == 255 {
        chroma = set_rnd_color_chroma4();
    }
    chroma
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

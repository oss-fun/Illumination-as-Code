use blinkt::Blinkt;
use rand::{thread_rng, Rng};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::{self, Write};
use std::usize;

#[derive(Serialize, Deserialize)]
pub struct Chroma {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}
#[derive(Serialize, Deserialize)]
pub struct Count {
    pub red: usize,
    pub green: usize,
    pub blue: usize,
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

pub fn blinkt_flash(
    chroma: &Chroma,
    blinkt: &mut Blinkt,
    numbers: &Vec<usize>,
) -> Result<(), Box<dyn Error>> {
    blinkt.set_pixel_rgbb(numbers[0], chroma.red, 0, 0, 0.1);
    blinkt.set_pixel_rgbb(numbers[1], 0, chroma.green, 0, 0.1);
    blinkt.set_pixel_rgbb(numbers[2], 0, 0, chroma.blue, 0.1);
    blinkt.set_pixel_rgbb(numbers[3], chroma.red, chroma.green, chroma.blue, 0.1);

    blinkt.show()?;
    Ok(())
}

pub fn input_chroma() -> Result<Chroma, Box<dyn Error>> {
    let mut chroma = Chroma {
        red: 0,
        green: 0,
        blue: 0,
    };

    // user input chroma R
    print!("Input R:");
    io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    chroma.red = parse_chroma(input)?;

    // user input chroma G
    print!("Input G:");
    io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    chroma.green = parse_chroma(input)?;

    // user input chroma B
    print!("Input B:");
    io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    chroma.blue = parse_chroma(input)?;

    println!("");

    if chroma.red == 0 && chroma.green == 0 && chroma.blue == 0 {
        return Err("Please Input number".into());
    } else {
        return Ok(chroma);
    }
}

fn parse_chroma(input: String) -> Result<u8, Box<dyn Error>> {
    match input.trim().parse()? {
        0 => Ok(0),
        1 => Ok(85),
        2 => Ok(170),
        3 => Ok(255),
        _ => Err("Please Input number 0~3".into()),
    }
}

impl Chroma {
    pub fn to_count(&self) -> Count {
        Count {
            red: chroma_to_count(self.red),
            green: chroma_to_count(self.green),
            blue: chroma_to_count(self.blue),
        }
    }
}

fn chroma_to_count(value: u8) -> usize {
    match value {
        0 => 0,
        85 => 1,
        170 => 2,
        255 => 3,
        _ => 0,
    }
}

pub async fn vm_up(chroma_answer: &Chroma, url: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let res = client
        .post(url.to_string() + "/vm_up")
        .json(&chroma_answer.to_count())
        .send()
        .await?;
    Ok(res.text().await?)
}

pub async fn start_receive_data(compute_time: usize, url: &str) -> Result<String, reqwest::Error> {
    println!("Start receive data!");
    let client = Client::new();
    let res = client
        .post(url.to_string() + "/start")
        .body(compute_time.to_string())
        .send()
        .await?;
    Ok(res.text().await?)
}

pub async fn get_chroma(compute_time: usize, url: &str) -> Result<Chroma, reqwest::Error> {
    let client = Client::new();
    let res = client
        .post(url.to_string() + "/get-chroma")
        .body(compute_time.to_string())
        .send()
        .await?;
    let count: Count = res.json().await?;
    Ok(Chroma {
        red: parse_chroma(count.red.to_string()).unwrap(),
        green: parse_chroma(count.green.to_string()).unwrap(),
        blue: parse_chroma(count.blue.to_string()).unwrap(),
    })
}

use blinkt::Blinkt;
use blinkt_raspi::{set_rnd_color_chroma4, Chroma};
use std::error::Error;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[warn(unused_assignments)]
fn main() -> Result<(), Box<dyn Error>> {
    let mut blinkt = Blinkt::new()?;
    //let url = "http://127.0.0.1:8080";
    let question_pins: Vec<usize> = vec![1, 3, 5, 7];
    let answer_pins: Vec<usize> = vec![0, 2, 4, 6];
    let mut chroma_question: Chroma;
    let mut chroma_answer: Chroma;

    loop {
        blinkt.clear();

        println!("Press Enter to Start!");
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        chroma_question = set_rnd_color_chroma4();
        blinkt_flash(&chroma_question, &mut blinkt, &question_pins)?;

        // user input chroma
        match input_chroma() {
            Ok(chroma) => {
                chroma_answer = chroma;
            }
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        }

        blinkt_flash(&chroma_question, &mut blinkt, &question_pins)?;
        println!(
            "Question: R:{} G:{} B:{}",
            chroma_question.red, chroma_question.green, chroma_question.blue
        );
        blinkt_flash(&chroma_answer, &mut blinkt, &answer_pins)?;
        println!(
            "Answer: R:{} G:{} B:{}",
            chroma_answer.red, chroma_answer.green, chroma_answer.blue
        );
        thread::sleep(Duration::from_millis(250));
        println!("");
    }
}

fn blinkt_flash(
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

fn input_chroma() -> Result<Chroma, Box<dyn Error>> {
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

use blinkt::Blinkt;
use blinkt_raspi::{blinkt_flash, input_chroma, set_rnd_color_chroma4, Chroma};
use std::error::Error;
use std::io::{self};
use std::thread;
use std::time::Duration;

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

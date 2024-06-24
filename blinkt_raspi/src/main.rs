use actix_rt::Runtime;
use blinkt::Blinkt;
use blinkt_raspi::{
    blinkt_flash, get_chroma, input_chroma, set_rnd_color_chroma4, start_receive_data, vm_up,
    Chroma,
};
use std::error::Error;
use std::io::{self};
use std::thread;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let rt = Runtime::new()?;
    rt.block_on(async {
        let mut blinkt = Blinkt::new()?;
        let local_url = "http://127.0.0.1:8080";
        let server_url = "http://192.168.1.1:8080";
        let question_pins: Vec<usize> = vec![1, 3, 5, 7];
        let answer_pins: Vec<usize> = vec![0, 2, 4, 6];
        let mut chroma_question: Chroma;
        let mut chroma_answer: Chroma;
        let mut chroma_computed: Chroma = Chroma {
            red: 0,
            green: 0,
            blue: 0,
        };

        loop {
            blinkt.clear();

            // start
            println!("Press Enter to Start!");
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            // create question chroma
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

            println!("vm up...");
            // vm up
            match vm_up(&chroma_answer, server_url).await {
                Ok(res) => {
                    println!("VM up: {}", res);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            }

            // start get answer chroma
            match start_receive_data(10, local_url).await {
                Ok(res) => {
                    println!("{}", res);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            }

            // get answer chroma
            match get_chroma(10, local_url).await {
                Ok(res) => {
                    chroma_computed.red = res.red;
                    chroma_computed.green = res.green;
                    chroma_computed.blue = res.blue;
                    println!("Computed: R:{} G:{} B:{}", res.red, res.green, res.blue);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    continue;
                }
            }

            // Checking answers
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
    })
}

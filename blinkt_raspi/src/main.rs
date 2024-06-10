use std::error::Error;
use std::thread;
use std::time::Duration;

use blinkt_raspi::{set_rnd_color_chroma4};

use rand::{thread_rng};
//use rand::seq::SliceRandom;

use blinkt::Blinkt;

fn main() -> Result<(), Box<dyn Error>> {
    let mut blinkt = Blinkt::new()?;
    loop {
        let mut rng = thread_rng();
        let (red, green, blue) = set_rnd_color_chroma4(&mut rng);
        println!("R:{:03},G:{:03},B:{:03}",red,blue,green);
        blinkt.set_pixel_rgbb(0, red, 0, 0, 0.1);
        blinkt.set_pixel_rgbb(2, 0, green, 0, 0.1);
        blinkt.set_pixel_rgbb(4, 0, 0, blue, 0.1);
        blinkt.set_pixel_rgbb(7, red, green, blue, 0.1);

        blinkt.show()?;

        thread::sleep(Duration::from_millis(250));
    }
}


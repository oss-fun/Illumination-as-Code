use std::error::Error;
use std::thread;
use std::time::Duration;

use blinkt_raspi::set_chroma_static;

use blinkt::Blinkt;

async fn main() -> Result<(), Box<dyn Error>> {
    let mut blinkt = Blinkt::new()?;
    let url = "http://127.00.1:8080";
    loop {
        match set_chroma_static(url).await {
            Ok((red, green, blue)) => {
                println!("R:{:03},G:{:03},B:{:03}", red, green, blue);
                blinkt.set_pixel_rgbb(0, red, 0, 0, 0.1);
                blinkt.set_pixel_rgbb(2, 0, green, 0, 0.1);
                blinkt.set_pixel_rgbb(4, 0, 0, blue, 0.1);
                blinkt.set_pixel_rgbb(7, red, green, blue, 0.1);

                blinkt.show()?;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }

        thread::sleep(Duration::from_millis(250));
    }
}

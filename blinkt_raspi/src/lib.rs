use rand::Rng;

pub fn set_rnd_color_chroma3(rng:&mut impl Rng) -> (u8, u8, u8) {
    let mut color: (u8, u8, u8) = (0, 0, 0);
    let chromas = [0,127,255];
    
    color.0 = chromas[rng.gen_range(0..chromas.len())];
    color.1 = chromas[rng.gen_range(0..chromas.len())];
    color.2 = chromas[rng.gen_range(0..chromas.len())];
    color
}

pub fn set_rnd_color_chroma4(rng:&mut impl Rng) -> (u8, u8, u8) {
    let mut color: (u8, u8, u8) = (0, 0, 0); 
    let chromas = [0,85,170,255];

    color.0 = chromas[rng.gen_range(0..chromas.len())];
    color.1 = chromas[rng.gen_range(0..chromas.len())];
    color.2 = chromas[rng.gen_range(0..chromas.len())];
    color
}
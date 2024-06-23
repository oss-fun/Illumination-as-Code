pub fn parse_count_to_chroma(data: &str, color: &str, time: usize) -> u8 {
    let count: usize = data.match_indices(color).count();

    let chroma: u8 = if (count / time * (time / 6) * 85) < 255 {
        (count / time * (time / 6) * 85) as u8
    } else {
        255
    };
    println!("R: count:{}, parsed:{}", count, chroma); // log
    chroma
}

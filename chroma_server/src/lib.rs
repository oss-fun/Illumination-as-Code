pub fn parse_count_to_chroma(data: &str, color: &str, time: usize) -> usize {
    let count: usize = data.match_indices(color).count();

    let chroma: usize = if (count * 10) / (5 * time) < 4 {
        (count * 10) / (5 * time)
    } else {
        3
    };
    println!("{}: count:{}", color, chroma); // log
    chroma
}

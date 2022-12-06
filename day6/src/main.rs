use advent_util::read_input;

const SIZE: usize = 26;

fn main() {
    let input = read_input(6).unwrap();

    let start_of_packet = search_for_start_of(&input, 4);
    let start_of_message = search_for_start_of(&input, 14);

    println!(
        "Symbols for processing before start-of-packet is found: {}",
        (start_of_packet + 4)
    );
    println!(
        "Symbols for processing before start-of-message is found: {}",
        (start_of_message + 14)
    );
}

fn search_for_start_of(input: &String, marker_length: u8) -> usize {
    let (index, _) = input
        .as_bytes()
        .windows(marker_length as usize)
        .enumerate()
        .find(|(_, signals)| is_begin_of_message(signals, marker_length))
        .unwrap();
    index
}

fn is_begin_of_message(signals: &[u8], marker_length: u8) -> bool {
    let mut characters = [false; SIZE];
    for signal in signals {
        characters[*signal as usize - 'a' as usize] = true;
    }
    characters
        .iter()
        .map(|has_char| if *has_char { 1 } else { 0 })
        .sum::<u8>()
        == marker_length
}

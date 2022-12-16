use crate::packets::PacketElement;
use advent_util::read_input;

mod packets;

fn main() {
    let packets_pairs = read_input(13)
        .unwrap()
        .split("\n\n")
        .map(|pair| pair.split_once("\n").unwrap())
        .map(|(f, s)| {
            (
                f.parse::<PacketElement>().unwrap(),
                s.parse::<PacketElement>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let result: usize = packets_pairs
        .iter()
        .enumerate()
        .map(|(i, (f, s))| (i, f < s))
        .filter(|(_, result)| *result)
        .map(|(i, _)| i + 1)
        .sum();

    println!("Sum of indices of pairs on the right order: {}", result);

    let mut packets = packets_pairs
        .iter()
        .flat_map(|(f, s)| [f, s])
        .collect::<Vec<_>>();

    let sep_2: PacketElement = "[[2]]".parse().unwrap();
    let sep_6: PacketElement = "[[6]]".parse().unwrap();

    packets.push(&sep_2);
    packets.push(&sep_6);

    packets.sort();

    let sep_2_index = packets.binary_search(&&sep_2).unwrap() + 1;
    let sep_6_index = packets.binary_search(&&sep_6).unwrap() + 1;

    println!("Decoder key is: {}", sep_2_index * sep_6_index);
}

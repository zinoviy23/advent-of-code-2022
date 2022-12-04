use std::fs::File;
use std::io::Read;

fn main() {
    let mut input_file = File::open("day1/input.txt")
        .expect("Download file with 'download.http'");
    let mut input = String::new();
    input_file.read_to_string(&mut input).expect("Cannot read");

    let elf_snacks = input.split("\n")
        .fold(vec![], |mut acc, line| {
            if line.is_empty() {
                acc.push(vec![]);
            } else {
                let snack: u32 = line.parse().expect(&format!("Wrong line {}", line));
                if let Some(current_elf) = acc.last_mut() {
                    current_elf.push(snack);
                } else {
                    acc.push(vec![snack]);
                }
            }
            acc
        });

    let mut elf_calories = elf_snacks.iter()
        .map(|elf| elf.iter().sum())
        .collect::<Vec<u32>>();
    elf_calories.sort_by(|calories1, calories2| calories2.cmp(calories1));
    let sum_of_most_3: u32 = elf_calories.iter().take(3).sum();

    println!("Max elf snack calories = {}", elf_calories[0]);
    println!("Sum of top 3 is {}", sum_of_most_3);
}

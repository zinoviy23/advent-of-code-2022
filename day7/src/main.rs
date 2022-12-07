use crate::commands::{CdArg, Command, Input, LsOutput};
use crate::files::{FileItem, FileTree};
use advent_util::read_input;

mod commands;
mod files;

const MAX_SIZE: u32 = 100000;
const TOTAL_SIZE: u32 = 70000000;
const NECESSARY_FREE_SPACE: u32 = 30000000;

fn main() {
    let input = read_input(7).unwrap();
    let input: Vec<Input> = input.lines().map(|line| line.parse().unwrap()).collect();

    let file_tree = build_file_tree(&input);

    let file_sizes = file_tree.traverse(|file, children_results| match file {
        FileItem::File(_, size) => *size,
        FileItem::Directory(_) => children_results.iter().sum(),
    });

    let sum_of_size: u32 = file_sizes
        .iter()
        .filter(|(file, size)| {
            if let FileItem::Directory(_) = file {
                *size <= MAX_SIZE
            } else {
                false
            }
        })
        .map(|(_, size)| size)
        .sum();

    let (_, root_size) = file_sizes[0];

    println!(
        "Sum of sizes of directories which size less than {}: {}",
        MAX_SIZE, sum_of_size
    );
    println!(
        "Total used space: {}, total free space: {}",
        root_size,
        TOTAL_SIZE - root_size
    );

    let min_folder_size_to_delete = file_sizes
        .iter()
        .filter(|(file, size)| {
            if let FileItem::Directory(_) = file {
                TOTAL_SIZE - root_size + size >= NECESSARY_FREE_SPACE
            } else {
                false
            }
        })
        .map(|(_, size)| *size)
        .min()
        .unwrap();

    println!(
        "Min folder size to delete: {}. This will produce free space: {}",
        min_folder_size_to_delete,
        TOTAL_SIZE - root_size + min_folder_size_to_delete
    );
}

fn build_file_tree(input: &Vec<Input>) -> FileTree {
    let mut file_tree_builder = FileTree::builder();

    for input_line in input {
        match input_line {
            Input::Command(cmd) => match cmd {
                Command::Cd(arg) => match arg {
                    CdArg::Parent => file_tree_builder.cd_parent(),
                    CdArg::Root => file_tree_builder.cd_root(),
                    CdArg::Dir(dir) => file_tree_builder.cd(dir),
                },
                Command::Ls => {}
            },
            Input::LsOutput(output) => match output {
                LsOutput::Dir(name) => file_tree_builder.mkdir(name),
                LsOutput::File(name, size) => file_tree_builder.touch(name, *size),
            },
        }
    }
    let file_tree = file_tree_builder.build();
    file_tree
}

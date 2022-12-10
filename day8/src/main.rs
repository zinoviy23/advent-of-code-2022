use advent_util::read_input;
use std::cmp::max;

fn main() {
    let trees = read_trees();
    if trees.is_empty() {
        return;
    }

    let width = trees[0].len();
    let height = trees.len();

    let max_trees_before = calculate_max_trees_before(&trees, width, height);

    let count_of_visible_trees = count_of_visible_tress(&trees, &max_trees_before);

    println!("Amount of trees: {}", width * height);
    println!("Amount of visible trees: {}", count_of_visible_trees);

    let max_scenic_score = highest_scenic_score(&trees, &max_trees_before, width, height);
    println!("Highest scenic score: {}", max_scenic_score);
}

fn highest_scenic_score(
    trees: &Vec<Vec<i8>>,
    max_trees_before: &Vec<Vec<MaxTreesBefore>>,
    width: usize,
    height: usize,
) -> usize {
    let mut max_score = 0usize;
    for i in 0..height {
        for j in 0..width {
            let score = calculate_score(trees, max_trees_before, width, height, i, j);
            max_score = max(max_score, score);
        }
    }

    max_score
}

fn calculate_score(
    trees: &Vec<Vec<i8>>,
    max_trees_before: &Vec<Vec<MaxTreesBefore>>,
    width: usize,
    height: usize,
    i: usize,
    j: usize,
) -> usize {
    let left: usize = {
        if max_trees_before[i][j].left() < trees[i][j] {
            j
        } else {
            (max_trees_before[i][j].left_pos()..j)
                .rev()
                .take_while(|neighbour| trees[i][j] > trees[i][*neighbour])
                .count()
                + 1
        }
    };
    let top: usize = {
        if max_trees_before[i][j].top() < trees[i][j] {
            i
        } else {
            (max_trees_before[i][j].top_pos()..i)
                .rev()
                .take_while(|neighbour| trees[i][j] > trees[*neighbour][j])
                .count()
                + 1
        }
    };
    let right: usize = {
        if max_trees_before[i][j].right() < trees[i][j] {
            width - 1 - j
        } else {
            (j + 1..max_trees_before[i][j].right_pos())
                .take_while(|neighbour| trees[i][j] > trees[i][*neighbour])
                .count()
                + 1
        }
    };
    let bottom: usize = {
        if max_trees_before[i][j].bottom() < trees[i][j] {
            height - 1 - i
        } else {
            (i + 1..max_trees_before[i][j].bottom_pos())
                .take_while(|neighbour| trees[i][j] > trees[*neighbour][j])
                .count()
                + 1
        }
    };
    let score = left * right * top * bottom;
    score
}

fn count_of_visible_tress(
    trees: &Vec<Vec<i8>>,
    max_trees_before: &Vec<Vec<MaxTreesBefore>>,
) -> usize {
    let count_of_visible_trees: usize = trees
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(j, tree)| max_trees_before[i][*j].is_tree_visible(**tree))
                .count()
        })
        .sum();
    count_of_visible_trees
}

fn calculate_max_trees_before(
    trees: &Vec<Vec<i8>>,
    width: usize,
    height: usize,
) -> Vec<Vec<MaxTreesBefore>> {
    let mut max_length_before = vec![vec![MaxTreesBefore::new(); width]; height];
    for i in 0..height {
        for j in 0..width {
            if i > 0 {
                let before = max_length_before[i - 1][j].clone();
                if before.top() <= trees[i - 1][j] {
                    max_length_before[i][j].set_top(trees[i - 1][j], i - 1);
                } else {
                    max_length_before[i][j].set_top(before.top(), before.top_pos());
                }
            }

            if j > 0 {
                let before = max_length_before[i][j - 1].clone();
                if before.left() <= trees[i][j - 1] {
                    max_length_before[i][j].set_left(trees[i][j - 1], j - 1);
                } else {
                    max_length_before[i][j].set_left(before.left(), before.left_pos());
                }
            }
        }
    }
    for i in (0..height).rev() {
        for j in (0..width).rev() {
            if i < height - 1 {
                let before = max_length_before[i + 1][j].clone();
                if before.bottom() <= trees[i + 1][j] {
                    max_length_before[i][j].set_bottom(trees[i + 1][j], i + 1);
                } else {
                    max_length_before[i][j].set_bottom(before.bottom(), before.bottom_pos());
                }
            }
            if j < width - 1 {
                let before = max_length_before[i][j + 1].clone();
                if before.right() <= trees[i][j + 1] {
                    max_length_before[i][j].set_right(trees[i][j + 1], j + 1);
                } else {
                    max_length_before[i][j].set_right(before.right(), before.right_pos());
                }
            }
        }
    }
    max_length_before
}

#[derive(Debug, Clone)]
struct MaxTreesBefore {
    max_values: [i8; 4],
    max_positions: [usize; 4],
}

impl MaxTreesBefore {
    fn new() -> Self {
        Self {
            max_values: [-1; 4],
            max_positions: [0; 4],
        }
    }

    fn top(&self) -> i8 {
        self.max_values[0]
    }

    fn left(&self) -> i8 {
        self.max_values[1]
    }

    fn right(&self) -> i8 {
        self.max_values[2]
    }

    fn bottom(&self) -> i8 {
        self.max_values[3]
    }

    fn set_top(&mut self, value: i8, position: usize) {
        self.max_values[0] = value;
        self.max_positions[0] = position;
    }

    fn set_left(&mut self, value: i8, position: usize) {
        self.max_values[1] = value;
        self.max_positions[1] = position;
    }

    fn set_right(&mut self, value: i8, position: usize) {
        self.max_values[2] = value;
        self.max_positions[2] = position;
    }

    fn set_bottom(&mut self, value: i8, position: usize) {
        self.max_values[3] = value;
        self.max_positions[3] = position;
    }

    fn top_pos(&self) -> usize {
        self.max_positions[0]
    }

    fn left_pos(&self) -> usize {
        self.max_positions[1]
    }

    fn right_pos(&self) -> usize {
        self.max_positions[2]
    }

    fn bottom_pos(&self) -> usize {
        self.max_positions[3]
    }

    fn is_tree_visible(&self, tree: i8) -> bool {
        self.max_values
            .iter()
            .any(|max_tree_before| *max_tree_before < tree)
    }
}

fn read_trees() -> Vec<Vec<i8>> {
    let trees = read_input(8)
        .unwrap()
        .lines()
        .map(|line| line.as_bytes())
        .map(|bytes| {
            bytes
                .iter()
                .map(|b| (*b - '0' as u8) as i8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    trees
}

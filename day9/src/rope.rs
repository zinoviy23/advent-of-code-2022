use advent_util::slices::foreach_windows;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Default)]
pub struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn move_towards(&mut self, direction: Direction) {
        match direction {
            Direction::Right => {
                self.x += 1;
            }
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Up => {
                self.y += 1;
            }
            Direction::Down => {
                self.y -= 1;
            }
        }
    }
}

#[derive(Debug)]
pub struct Rope {
    knots: Vec<Pos>,
}

impl Rope {
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        Self {
            knots: vec![Pos::default(); size],
        }
    }

    fn head_mut(&mut self) -> &mut Pos {
        &mut self.knots[0]
    }

    fn knots_mut(&mut self) -> &mut [Pos] {
        &mut self.knots
    }

    fn tail(&self) -> Pos {
        *self.knots.last().unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
    direction: Direction,
    steps: usize,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.strip_prefix("R ")
            .map(|rest| (Direction::Right, rest))
            .or_else(|| s.strip_prefix("L ").map(|rest| (Direction::Left, rest)))
            .or_else(|| s.strip_prefix("U ").map(|rest| (Direction::Up, rest)))
            .or_else(|| s.strip_prefix("D ").map(|rest| (Direction::Down, rest)))
            .ok_or_else(|| format!("Unexpected move command: {}", s))
            .and_then(|(direction, rest)| {
                rest.parse::<usize>()
                    .map_err(|msg| format!("Cannot parse steps: {}", msg))
                    .map(|steps| Self { direction, steps })
            })
    }
}

#[derive(Debug)]
pub struct RopeMover {
    rope: Rope,
    tail_trail: Vec<Pos>,
}

impl RopeMover {
    pub fn new(knots_amount: usize) -> Self {
        Self {
            rope: Rope::new(knots_amount),
            tail_trail: vec![Pos::default()], // at start tail already was in (0, 0)
        }
    }

    pub fn move_head(&mut self, move_info: Move) {
        for _ in 0..move_info.steps {
            self.move_head_once(move_info.direction);
        }
    }

    fn move_head_once(&mut self, direction: Direction) {
        self.rope.head_mut().move_towards(direction);

        foreach_windows::<_, 2, _>(self.rope.knots_mut(), |[prev, current]| {
            Self::move_according_knot_previous(*prev, current)
        });

        self.tail_trail.push(self.rope.tail());
    }

    fn move_according_knot_previous(prev: Pos, current: &mut Pos) {
        let x_diff = prev.x - current.x;
        let y_diff = prev.y - current.y;
        if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
            return;
        }

        if x_diff.abs() > 1 {
            current.x += x_diff.signum();
            if y_diff != 0 {
                current.y += y_diff.signum();
            }
        } else if y_diff.abs() > 1 {
            current.y += y_diff.signum();
            if x_diff != 0 {
                current.x += x_diff.signum();
            }
        }
    }

    pub fn tail_trail(&self) -> &[Pos] {
        self.tail_trail.as_slice()
    }
}

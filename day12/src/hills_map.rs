use array2d::Array2D;
use std::cmp::Ordering;
use std::collections::VecDeque;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub struct Location {
    row: usize,
    column: usize,
    height: u8,
}

impl Location {
    fn as_index(&self) -> (usize, usize) {
        (self.row, self.column)
    }
}

#[derive(Debug)]
pub struct HillsMap {
    hills: Array2D<Location>,
}

impl HillsMap {
    pub fn parse_map(s: &str) -> (Self, Location, Location) {
        let mut hills = vec![];
        let mut start = None;
        let mut end = None;
        for (row, line) in s.trim().split("\n").enumerate() {
            let mut current_row = vec![];
            for (column, height_char) in line.chars().enumerate() {
                let height = match height_char {
                    'S' => {
                        start = Some(Location {
                            row,
                            column,
                            height: 0,
                        });
                        0
                    }
                    'E' => {
                        let height = 'z' as u8 - 'a' as u8;
                        end = Some(Location {
                            row,
                            column,
                            height,
                        });
                        height
                    }
                    _ => height_char as u8 - 'a' as u8,
                };

                current_row.push(Location {
                    row,
                    column,
                    height,
                })
            }
            hills.push(current_row);
        }

        let hills = Array2D::from_rows(&hills).unwrap();

        let start = start.unwrap();
        let end = end.unwrap();

        (Self { hills }, start, end)
    }

    fn bfs<'a, F, I>(&'a self, from: Location, neighbours: F) -> Array2D<Option<(usize, Location)>>
    where
        I: Iterator<Item = Location> + 'a,
        F: Fn(Location) -> I,
    {
        let mut distance =
            Array2D::filled_with(None, self.hills.column_len(), self.hills.row_len());

        distance[from.as_index()] = Some((0, from));

        let mut queue = VecDeque::from([(from, 0)]);
        while let Some((current, current_distance)) = queue.pop_front() {
            for neighbour in neighbours(current) {
                let neighbour_distance = distance[neighbour.as_index()];
                match neighbour_distance {
                    None => {
                        let new_distance = current_distance + 1;
                        distance[neighbour.as_index()] = Some((new_distance, current));
                        queue.push_back((neighbour, new_distance));
                    }
                    Some((neighbour_distance, _)) if neighbour_distance > current_distance + 1 => {
                        let new_distance = current_distance + 1;
                        distance[neighbour.as_index()] = Some((new_distance, current));
                        queue.push_back((neighbour, new_distance));
                    }
                    _ => {}
                }
            }
        }

        distance
    }

    pub fn find_path(&self, from: Location, to: Location) -> Option<(usize, Vec<Location>)> {
        let distance = self.bfs(from, |location| self.neighbours(location));

        if let Some((len, _)) = distance[to.as_index()] {
            Some((len, Self::restore_path(&distance, to)))
        } else {
            None
        }
    }

    pub fn find_shortest_path_from_lowest(
        &self,
        to: Location,
    ) -> Option<(Location, usize, Vec<Location>)> {
        let distance = self.bfs(to, |location| self.backward_neighbours(location));

        let mut min: Option<(usize, Location)> = None;
        for row in self.hills.rows_iter() {
            for location in row {
                if *location != to && location.height == 0 {
                    let (current_distance, _) = if let Some(dist) = distance[location.as_index()] {
                        dist
                    } else {
                        continue;
                    };
                    if let Some((min_distance, _)) = min {
                        if min_distance > current_distance {
                            min = Some((current_distance, *location));
                        }
                    } else {
                        min = Some((current_distance, *location));
                    }
                }
            }
        }

        if let Some((min_distance, from)) = min {
            let mut path = Self::restore_path(&distance, from);
            path.reverse();
            Some((from, min_distance, path))
        } else {
            None
        }
    }

    fn possible_neighbours<'a>(
        &'a self,
        location: Location,
    ) -> impl Iterator<Item = Location> + 'a {
        [(-1isize, 0isize), (1, 0), (0, 1), (0, -1)]
            .iter()
            .filter_map(move |(d_row, d_column)| {
                let row = match location.row.checked_add_signed(*d_row) {
                    Some(row) if row < self.hills.column_len() => row,
                    _ => return None,
                };
                let column = match location.column.checked_add_signed(*d_column) {
                    Some(column) if column < self.hills.row_len() => column,
                    _ => return None,
                };
                Some(self.hills[(row, column)])
            })
    }

    fn neighbours<'a>(&'a self, location: Location) -> impl Iterator<Item = Location> + 'a {
        self.possible_neighbours(location)
            .filter(move |neighbour| neighbour.height <= location.height + 1)
    }

    fn backward_neighbours<'a>(
        &'a self,
        location: Location,
    ) -> impl Iterator<Item = Location> + 'a {
        self.possible_neighbours(location)
            .filter(move |neighbour| location.height <= neighbour.height + 1)
    }

    fn restore_path(distance: &Array2D<Option<(usize, Location)>>, to: Location) -> Vec<Location> {
        let mut current = to;
        let mut path = vec![current];
        while let Some((_, previous)) = distance[current.as_index()] {
            if previous == current {
                break;
            }
            path.push(previous);

            current = previous;
        }
        path.reverse();
        path
    }

    pub fn render_path(&self, path: &[Location]) -> String {
        let mut render = Array2D::filled_with('.', self.hills.column_len(), self.hills.row_len());
        for window in path.windows(2) {
            let current = window[0];
            let next = window[1];

            match (current.column.cmp(&next.column), current.row.cmp(&next.row)) {
                (Ordering::Greater, _) => render[current.as_index()] = '<',
                (Ordering::Less, _) => render[current.as_index()] = '>',
                (_, Ordering::Greater) => render[current.as_index()] = '^',
                (_, Ordering::Less) => render[current.as_index()] = 'v',
                _ => {}
            }
        }

        if let Some(last) = path.last() {
            render[last.as_index()] = 'E'
        }

        render
            .rows_iter()
            .map(|row| String::from_iter(row))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

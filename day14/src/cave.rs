use array2d::Array2D;
use bevy::math::vec3;
use bevy::prelude::shape::{Cube, UVSphere};
use bevy::prelude::{
    info, Assets, Color, Commands, Component, Entity, Mesh, PbrBundle, Query, ResMut,
    StandardMaterial, Transform,
};
use std::str::FromStr;

#[derive(Debug)]
struct Wall {
    parts_path: Vec<(isize, isize)>,
}

impl FromStr for Wall {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts_path = s
            .split(" -> ")
            .map(|coord| coord.split_once(",").unwrap())
            .map(|(x, y)| (x.parse::<isize>().unwrap(), y.parse::<isize>().unwrap()))
            .collect::<Vec<_>>();
        Ok(Wall { parts_path })
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum CaveChunk {
    Air,
    Wall,
    Sand,
}

impl CaveChunk {
    fn render(&self) -> char {
        match self {
            CaveChunk::Air => '.',
            CaveChunk::Wall => '#',
            CaveChunk::Sand => 'o',
        }
    }
}

#[derive(Debug, Component)]
pub struct Cave {
    visible_rect: Array2D<CaveChunk>,
    rect_x_shift: usize,
    filled_with_sand: bool,
}

const SIZE: f32 = 0.2;

impl Cave {
    pub fn render(&self) -> String {
        self.visible_rect
            .rows_iter()
            .map(|row| String::from_iter(row.map(|chunk| chunk.render())))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn table_coord_to_world(&self, (row, column): (usize, usize)) -> (f32, f32) {
        let x_shift = self.visible_rect.row_len() as f32 / 2.;
        let y_shift = self.visible_rect.column_len() as f32 / 2.;

        (
            (column as f32 - x_shift) * SIZE,
            (y_shift - row as f32) * SIZE,
        )
    }

    fn move_sand(&mut self, sand: &mut MovingSand) -> MoveStatus {
        for option in sand.move_options() {
            if let Some((row, column)) = option.make_step() {
                if row >= self.visible_rect.column_len()
                    || column >= self.visible_rect.row_len() + self.rect_x_shift
                {
                    return MoveStatus::Out;
                }
                if self.visible_rect[(row, column - self.rect_x_shift)] == CaveChunk::Air {
                    sand.row = row;
                    sand.column = column;
                    return MoveStatus::Success;
                }
            } else {
                return MoveStatus::Out;
            }
        }

        self.visible_rect[(sand.row, sand.column - self.rect_x_shift)] = CaveChunk::Sand;

        MoveStatus::Stop
    }

    pub fn sand_count(&self) -> usize {
        self.visible_rect
            .rows_iter()
            .map(|row| row.filter(|chunk| **chunk == CaveChunk::Sand).count())
            .sum()
    }

    pub fn fill_with_sand(&mut self) {
        if self.filled_with_sand {
            return;
        }
        'outer: loop {
            let mut sand = MovingSand::default();
            loop {
                match self.move_sand(&mut sand) {
                    MoveStatus::Out => {
                        self.filled_with_sand = true;
                        break 'outer;
                    }
                    MoveStatus::Stop => {
                        break;
                    }
                    MoveStatus::Success => {}
                }
            }
        }
    }
}

impl FromStr for Cave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let walls = s
            .trim()
            .split("\n")
            .map(|line| line.parse::<Wall>().unwrap())
            .collect::<Vec<_>>();

        let mut min_x = None;
        let mut max_x = None;
        let mut max_y = None;

        for wall in walls.iter() {
            for (x, y) in wall.parts_path.iter() {
                if let Some(min_x_value) = min_x {
                    if min_x_value > x {
                        min_x = Some(x);
                    }
                } else {
                    min_x = Some(x);
                }

                if let Some(max_x_value) = max_x {
                    if max_x_value < x {
                        max_x = Some(x);
                    }
                } else {
                    max_x = Some(x);
                }

                if let Some(max_y_value) = max_y {
                    if max_y_value < y {
                        max_y = Some(y);
                    }
                } else {
                    max_y = Some(y);
                }
            }
        }

        let max_x = *max_x.unwrap() as usize;
        let max_y = *max_y.unwrap() as usize;
        let min_x = *min_x.unwrap() as usize;

        let mut visible_rect = Array2D::filled_with(CaveChunk::Air, max_y + 1, max_x - min_x + 1);

        for wall in walls.iter() {
            for window in wall.parts_path.as_slice().windows(2) {
                let (from_x, from_y) = window[0];
                let (to_x, to_y) = window[1];
                let d_x = (to_x - from_x).signum();
                let d_y = (to_y - from_y).signum();

                let mut current_x = from_x;
                let mut current_y = from_y;

                while current_x != to_x || current_y != to_y {
                    let coord_x = current_x as usize - min_x;
                    let coord_y = current_y as usize;
                    visible_rect[(coord_y, coord_x)] = CaveChunk::Wall;

                    current_x += d_x;
                    current_y += d_y;
                }

                let coord_x = to_x as usize - min_x;
                let coord_y = to_y as usize;
                visible_rect[(coord_y, coord_x)] = CaveChunk::Wall;
            }
        }

        Ok(Cave {
            visible_rect,
            rect_x_shift: min_x,
            filled_with_sand: false,
        })
    }
}

pub fn render_cave(
    mut commands: Commands,
    caves: Query<&Cave>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(Cube { size: SIZE }));
    let material = materials.add(StandardMaterial {
        metallic: 0.,
        reflectance: 0.,
        ..Color::rgb(0.9, 0.4, 0.4).into()
    });

    for cave in caves.iter() {
        info!("rendering cave");

        for (row_index, row) in cave.visible_rect.rows_iter().enumerate() {
            for (column_index, chunk) in row.enumerate() {
                let (x, y) = cave.table_coord_to_world((row_index, column_index));
                if let CaveChunk::Wall = chunk {
                    commands.spawn(PbrBundle {
                        transform: Transform::from_translation(vec3(x, y, 0.)),
                        mesh: mesh.clone(),
                        material: material.clone(),
                        ..Default::default()
                    });
                }
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct MovingSand {
    row: usize,
    column: usize,
}

impl Default for MovingSand {
    fn default() -> Self {
        Self {
            row: 0,
            column: 500,
        }
    }
}

#[derive(Clone, Copy)]
enum MoveOptions {
    Down((usize, usize)),
    Left((usize, usize)),
    Right((usize, usize)),
}

enum MoveStatus {
    Out,
    Stop,
    Success,
}

impl MovingSand {
    fn move_options(&self) -> impl Iterator<Item = MoveOptions> {
        [
            MoveOptions::Down((self.row, self.column)),
            MoveOptions::Left((self.row, self.column)),
            MoveOptions::Right((self.row, self.column)),
        ]
        .into_iter()
    }
}

impl MoveOptions {
    fn make_step(&self) -> Option<(usize, usize)> {
        match self {
            MoveOptions::Down((row, column)) => Some((*row + 1, *column)),
            MoveOptions::Left((row, column)) => column
                .checked_add_signed(-1)
                .map(|column| (*row + 1, column)),
            MoveOptions::Right((row, column)) => Some((*row + 1, *column + 1)),
        }
    }
}

pub fn move_sand(
    mut commands: Commands,
    mut caves: Query<&mut Cave>,
    mut moving_sand: Query<(Entity, &mut MovingSand, &mut Transform)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for mut cave in caves.iter_mut() {
        if cave.filled_with_sand {
            continue;
        }

        let mut any = false;
        let mut need_new_sand = false;
        for (entity, mut sand, mut transform) in moving_sand.iter_mut() {
            any = true;
            match cave.move_sand(&mut sand) {
                MoveStatus::Out => {
                    commands.entity(entity).despawn();

                    info!("Cannot add new sand!");
                    info!("Sand count: {}", cave.sand_count());
                    cave.filled_with_sand = true;
                }
                MoveStatus::Stop => {
                    commands.entity(entity).remove::<MovingSand>();
                    need_new_sand = true;
                }
                MoveStatus::Success => {
                    let (x, y) =
                        cave.table_coord_to_world((sand.row, sand.column - cave.rect_x_shift));
                    transform.translation = vec3(x, y, SIZE);
                }
            }
        }
        if !any || need_new_sand {
            let (x, y) = cave.table_coord_to_world((0, 500 - cave.rect_x_shift));
            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(UVSphere {
                        radius: 0.2,
                        sectors: 4,
                        stacks: 4,
                    })),
                    material: materials.add(StandardMaterial {
                        metallic: 0.,
                        reflectance: 0.,
                        ..Color::rgb(0.4, 0.9, 0.4).into()
                    }),
                    transform: Transform::from_translation(vec3(x, y, SIZE)),
                    ..Default::default()
                },
                MovingSand::default(),
            ));
        }
    }
}

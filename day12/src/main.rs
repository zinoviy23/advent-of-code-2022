use crate::hills_map::HillsMap;
use advent_util::read_input;

mod hills_map;

fn main() {
    let input = read_input(12).unwrap();
    let (map, start, end) = HillsMap::parse_map(&input);
    let path = map.find_path(start, end);

    if let Some((length, path)) = path {
        println!("Path length from S to E: {}", length);
        println!("Path:");
        println!("{}", map.render_path(&path));
    } else {
        println!("Error!! Cannot find path!")
    }

    let min_hiking_start = map.find_shortest_path_from_lowest(end);
    if let Some((from, distance, path)) = min_hiking_start {
        println!("Path length from miminal S {:?} to E: {}", from, distance);
        println!("Path:");
        println!("{}", map.render_path(&path));
    } else {
        println!("Error! Cannot find minimal start!")
    }
}

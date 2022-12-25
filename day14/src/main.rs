use crate::cave::{
    fill_cave_with_sand_completely, move_sand, render_cave, Cave, CaveCache, CaveStatistics,
};
use advent_util::read_input;
use bevy::prelude::{
    App, Camera2dBundle, Commands, CoreStage, Msaa, Plugin, PluginGroup, Query, ResMut,
    StartupStage, SystemStage, WindowDescriptor, WindowPlugin,
};
use bevy::time::FixedTimestep;
use bevy::DefaultPlugins;
use std::env;

mod cave;

fn main() {
    App::new()
        .add_startup_system(read_cave)
        .add_plugin(CavePlugin)
        .run();
}

struct CavePlugin;

impl Plugin for CavePlugin {
    fn build(&self, app: &mut App) {
        if env::args().find(|arg| arg == "--render").is_some() {
            app.insert_resource(Msaa { samples: 4 })
                .add_plugins(DefaultPlugins.set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Caves".to_string(),
                        width: 800.,
                        height: 600.,
                        ..Default::default()
                    },
                    ..Default::default()
                }))
                .add_startup_system(setup_camera)
                .add_startup_system_to_stage(StartupStage::PostStartup, render_cave)
                .add_stage_after(
                    CoreStage::Update,
                    "fixed_update",
                    SystemStage::parallel()
                        .with_run_criteria(FixedTimestep::step(0.0001))
                        .with_system(move_sand),
                )
                .add_system(fill_cave_with_sand_completely)
                // .add_plugin(FrameTimeDiagnosticsPlugin::default())
                // .add_plugin(LogDiagnosticsPlugin::default())
                // .add_plugin(EntityCountDiagnosticsPlugin::default())
                .insert_resource(CaveStatistics::new())
                .insert_resource(CaveCache::default());
        } else {
            app.insert_resource(CaveStatistics::new())
                .add_stage(
                    "Begin",
                    SystemStage::single_threaded().with_system(debug_println_cave),
                )
                .add_stage_after(
                    "Begin",
                    "Fill",
                    SystemStage::single_threaded().with_system(fill_with_sand),
                )
                .add_stage_after(
                    "Fill",
                    "Result Print",
                    SystemStage::single_threaded().with_system(debug_println_cave),
                )
                .add_stage_after(
                    "Result Print",
                    "Fill Completely",
                    SystemStage::single_threaded().with_system(fill_with_sand_completely),
                )
                .add_stage_after(
                    "Fill Completely",
                    "Result Result print",
                    SystemStage::single_threaded().with_system(debug_println_cave),
                );
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn read_cave(mut commands: Commands) {
    let cave: Cave = read_input(14).unwrap().parse().unwrap();
    commands.spawn(cave);
}

fn debug_println_cave(query: Query<&Cave>) {
    for cave in query.iter() {
        println!("Rendered cave:\n{}", cave.render());
    }
}

fn fill_with_sand(mut caves: Query<&mut Cave>, mut cave_statistics: ResMut<CaveStatistics>) {
    for mut cave in caves.iter_mut() {
        cave.fill_with_sand();

        cave_statistics.without_floor = cave.sand_count();
        println!("Sand count: {}", cave_statistics.without_floor);
    }
}

fn fill_with_sand_completely(
    mut caves: Query<&mut Cave>,
    mut cave_statistics: ResMut<CaveStatistics>,
) {
    for mut cave in caves.iter_mut() {
        cave.fill_completely();

        cave_statistics.at_all = cave.sand_total();

        println!("Sand count: {}", cave_statistics.at_all);
        println!(
            "New sand: {}",
            cave_statistics.at_all - cave_statistics.without_floor
        );
    }
}

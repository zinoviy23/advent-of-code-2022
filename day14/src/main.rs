use crate::cave::{move_sand, render_cave, Cave};
use advent_util::read_input;
use bevy::math::vec3;
use bevy::prelude::{
    App, Camera3dBundle, Commands, CoreStage, DirectionalLight, DirectionalLightBundle, Msaa,
    Plugin, PluginGroup, Quat, Query, StartupStage, SystemStage, Transform, WindowDescriptor,
    WindowPlugin,
};
use bevy::time::FixedTimestep;
use bevy::DefaultPlugins;
use std::env;
use std::f32::consts::PI;

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
                        .with_run_criteria(FixedTimestep::step(0.001))
                        .with_system(move_sand),
                );
        } else {
            app.add_stage(
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
            );
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(vec3(0., 0., 42f32)),
        ..Default::default()
    });
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight::default(),
        transform: Transform {
            translation: vec3(0., 2., 0.),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..Default::default()
        },
        ..Default::default()
    });
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

fn fill_with_sand(mut caves: Query<&mut Cave>) {
    for mut cave in caves.iter_mut() {
        cave.fill_with_sand();

        println!("Sand count: {}", cave.sand_count());
    }
}

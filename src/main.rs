use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_prototype_debug_lines::*;

pub mod barrel;
pub mod player;
pub mod world;

use barrel::*;
use player::*;
use world::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanOrbitCameraPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(DebugLinesPlugin::default())
        .add_startup_system(setup_world)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_barrel)
        .add_system(player_movement)
        .run();
}

fn setup_camera(mut cmds: Commands) {
    cmds.spawn((Camera3dBundle::default(), PanOrbitCamera::default()));
}

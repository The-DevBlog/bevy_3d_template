use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub mod world;

use world::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PanOrbitCameraPlugin)
        .add_startup_system(setup_world)
        .add_startup_system(setup_camera)
        .run();
}

fn setup_camera(mut cmds: Commands) {
    cmds.spawn((Camera3dBundle::default(), PanOrbitCamera::default()));
}

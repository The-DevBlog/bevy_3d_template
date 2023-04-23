use bevy::prelude::*;

pub fn setup_world(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: 50.0,
                ..default()
            })),
            material: materials.add(Color::DARK_GREEN.into()),
            ..default()
        },
        Name::new("Ground"),
    );

    let light = (
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: false,
                ..default()
            },
            transform: Transform {
                rotation: Quat::from_rotation_x(4.0),
                ..default()
            },
            ..default()
        },
        Name::new("Light"),
    );

    cmds.spawn(floor);
    cmds.spawn(light);
}

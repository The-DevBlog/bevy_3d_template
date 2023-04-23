use bevy::prelude::*;

const EXPLOSION_RADIUS: f32 = 6.0;

pub fn spawn_barrel(
    mut cmds: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let explosion_radius = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: EXPLOSION_RADIUS,
                height: 0.5,
                ..default()
            })),
            material: materials.add(Color::rgba(1.0, 0.14, 0.14, 0.2).into()),
            ..default()
        },
        Name::new("Explosion Radius"),
    );

    let barrel = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 1.0,
                height: 3.0,
                ..default()
            })),
            transform: Transform {
                translation: Vec3::new(-20.0, 1.5, -20.0),
                ..default()
            },
            material: materials.add(Color::RED.into()),
            ..default()
        },
        Name::new("Barrel"),
    );

    cmds.spawn(barrel).with_children(|parent| {
        parent.spawn(explosion_radius);
    });
}

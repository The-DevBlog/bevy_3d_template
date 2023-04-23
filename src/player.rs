use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut cmds: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let player = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 1.0,
                ..default()
            })),
            transform: Transform {
                translation: Vec3::new(20.0, 1.5, 20.0),
                ..default()
            },
            material: materials.add(Color::BLUE.into()),
            ..default()
        },
        Name::new("Player"),
        Player,
    );

    cmds.spawn(player);
}

pub fn player_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut player_q: Query<&mut Transform, With<Player>>,
) {
    for mut trans in player_q.iter_mut() {
        let speed = 15.0;
        let mut direction = Vec3::ZERO;

        // forward
        if keys.pressed(KeyCode::W) {
            direction.z -= 1.0;
        }

        // back
        if keys.pressed(KeyCode::S) {
            direction.z += 1.0;
        }

        // left
        if keys.pressed(KeyCode::A) {
            direction.x -= 1.0;
        }

        // right
        if keys.pressed(KeyCode::D) {
            direction.x += 1.0
        }

        direction.y = 0.0;
        trans.translation += speed * direction * time.delta_seconds();
    }
}

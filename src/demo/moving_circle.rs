use bevy::{
    prelude::*,
    sprite::{
        MaterialMesh2dBundle,
        Mesh2dHandle
    },
};

#[derive(Component)]
struct Player {
    speed: f32,
}

pub struct MovingCirclePlugin;

impl Plugin for MovingCirclePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_circle)
            .add_systems(Update, player_movement_system);
    }
}

fn setup_circle(
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2dBundle::default());
    let color = Color::srgb(0.0, 0.0, 1.0);

    commands.spawn((MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle::new(
            50.1
        ))),
        material: materials.add(color),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    },
        Player {
            speed: 500.0,
        })
    );
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    let (player, mut transform) = query.single_mut();

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= player.speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        transform.translation.x += player.speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        transform.translation.y += player.speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= player.speed * time.delta_seconds();
    }
}

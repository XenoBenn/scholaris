use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule::default())),
        material: materials.add(Color::WHITE.into()),
        transform: Transform::from_xyz(0.0, 1.0, 0.0),
        ..Default::default()
    };
    commands.spawn(player);
}
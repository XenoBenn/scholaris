use bevy::prelude::*;

pub struct LightPlugin;

impl Plugin for LightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_lights);
    }
}



fn spawn_lights(mut commands: Commands) {
    let light = DirectionalLightBundle {
        directional_light: DirectionalLight { 
            ..default() 
        },
        transform: Transform::from_xyz(1.0, 1.0, 1.0),
        ..default()
    };
    commands.spawn(light);
}
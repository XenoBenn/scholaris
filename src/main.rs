use bevy::prelude::*;

mod player;
mod camera;
mod lights;
mod terrain;

use player::PlayerPlugin;
use camera::CameraPlugin;
use lights::LightPlugin;
use terrain::TerrainPlugin;


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, CameraPlugin, LightPlugin, TerrainPlugin))
        .run();
}


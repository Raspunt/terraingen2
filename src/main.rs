use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;

mod terrain;
mod storage;
use terrain::render::TerrainGeneratorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(TerrainGeneratorPlugin)
        .run();
}

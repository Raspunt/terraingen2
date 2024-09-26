//! A simple 3D scene with light shining over a cube sitting on a plane.

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;

mod game;

use game::chunks::ChunkPlugin;
use game::render::RenderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(PlayerPlugin)
        .add_plugins(ChunkPlugin)
        .add_plugins(RenderPlugin)
        .run();
}

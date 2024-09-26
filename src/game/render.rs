use bevy::prelude::*;
use bevy_flycam::FlyCam;

use super::chunks::ChunkRenderRadius;
use super::chunks::ChunkSize;
use super::chunks::ChunkStorage;

fn render_chunks(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut chunk_storage: ResMut<ChunkStorage>,
    chunk_render_radius: Res<ChunkRenderRadius>,
    chunk_size: Res<ChunkSize>,
    player: Query<&GlobalTransform, (With<FlyCam>, Changed<GlobalTransform>)>,
) {
    if let Ok(ply) = player.get_single() {
        let player_position = ply.translation();
        for chunk in &mut chunk_storage.chunks {
            {
                let chunk_position = chunk.world_position;
                let distance_to_chunk = (chunk_position - player_position).length();

                if distance_to_chunk > chunk_render_radius.radius {
                    if chunk.is_visible {
                        for entity in &chunk.rendered_cubes {
                            commands.entity(*entity).despawn();
                        }
                        chunk.rendered_cubes.clear();
                    }
                    chunk.is_visible = false;
                    continue;
                }

                if !chunk.is_visible {
                    for x in -chunk_size.x..chunk_size.x {
                        for z in -chunk_size.z..chunk_size.z {
                            let cube_position =
                                chunk_position + Vec3::new(x as f32 / 2.0, 0.0, z as f32 / 2.0);

                            let cube_entity = commands
                                .spawn(PbrBundle {
                                    mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
                                    material: materials.add(StandardMaterial {
                                        base_color: Color::rgb(0.0, 0.0, 0.6),
                                        ..Default::default()
                                    }),
                                    transform: Transform::from_translation(cube_position),
                                    ..Default::default()
                                })
                                .id();
                            chunk.rendered_cubes.push(cube_entity);
                        }
                    }
                    chunk.is_visible = true;
                }
            }
        }
    }
}

fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

pub struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_light);
        app.add_systems(Update, render_chunks);
    }
}

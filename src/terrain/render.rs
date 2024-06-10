use bevy::prelude::*;

use crate::{storage::ChunkMap, terrain::heightmap_generator::HeightmapGenerator};

use super::{
    chanks::{ChanksMakerPlugin, Chunk, ChunkShape, CurrentLocalPlayerChunk, CHUNK_LENGTH},
    voxel::Voxel,
};

#[derive(Resource)]
pub struct PlayerPositionHistory {
    position: Vec<IVec3>,
}

#[derive(Resource)]
pub struct PreviousHeightMap {
    hmap: Vec<Vec<f32>>,
}

fn world_render(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    chunks: Query<(Entity, &Chunk), Added<Chunk>>,
    player_pos: Res<CurrentLocalPlayerChunk>,
    mut player_position_history: ResMut<PlayerPositionHistory>,
    mut previous_heightmap: ResMut<PreviousHeightMap>,
) {
    let cube = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
    let material_handle = materials.add(StandardMaterial {
        base_color: Color::rgb(0.0, 0.0, 0.6),
        ..Default::default()
    });

    if !player_position_history
        .position
        .contains(&player_pos.chunk_min)
    {
        player_position_history.position.push(player_pos.chunk_min);

        let mut hmap = HeightmapGenerator::new();
        let heightmap: Vec<Vec<f32>> = hmap
            .generate_heightmap(
                CHUNK_LENGTH.try_into().unwrap(),
                CHUNK_LENGTH.try_into().unwrap(),
                Some(&previous_heightmap.hmap),
            )
            .to_vec();


        println!("creating cubes !!!");

        for x in 0..CHUNK_LENGTH {
            for z in 0..CHUNK_LENGTH {
                let height_value = heightmap[z as usize][x as usize];

                commands.spawn(PbrBundle {
                    mesh: cube.clone(),
                    material: material_handle.clone(),
                    transform: Transform::from_xyz(
                        x as f32 + player_pos.chunk_min.x as f32,
                        height_value,
                        z as f32 + player_pos.chunk_min.z as f32,
                    ),
                    ..Default::default()
                });
            }
        }
        previous_heightmap.hmap = heightmap.clone();

    }
}
pub struct TerrainGeneratorPlugin;
impl Plugin for TerrainGeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkMap::<Voxel, ChunkShape>::new(ChunkShape {}));
        app.insert_resource(PlayerPositionHistory {
            position: vec![IVec3::ZERO],
        });
        app.insert_resource(PreviousHeightMap { hmap: vec![] });
        app.add_plugins(ChanksMakerPlugin);
        app.add_systems(Update, world_render);
    }
}

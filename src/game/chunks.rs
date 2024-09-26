use bevy::prelude::*;

use super::heightmap::Heightmap;

#[derive(Resource)]
pub struct WorldSize {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Resource)]
pub struct ChunkSize {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Resource)]
pub struct ChunkRenderRadius {
    pub radius: f32,
}

pub struct Chunk {
    pub world_position: Vec3,
    pub is_visible: bool,
    pub rendered_cubes: Vec<Entity>,
}

#[derive(Resource)]
pub struct ChunkStorage {
    pub chunks: Vec<Chunk>,
    pub num_chunks_x: i32,
    pub num_chunks_y: i32,
    pub num_chunks_z: i32,
    pub hmap: Vec<Vec<f32>>,
}

fn create_chunks(
    world_size: Res<WorldSize>,
    chunk_size: Res<ChunkSize>,
    mut chunk_storage: ResMut<ChunkStorage>,
) {
    // Количество чанков
    let num_chunks_x = world_size.x / chunk_size.x;
    let num_chunks_y = world_size.y / chunk_size.y;
    let num_chunks_z = world_size.z / chunk_size.z;

    println!("x chunks count: {}", num_chunks_x);
    println!("y chunks count: {}", num_chunks_y);
    println!("z chunks count: {}", num_chunks_z);

    chunk_storage.num_chunks_x = num_chunks_x;
    chunk_storage.num_chunks_y = num_chunks_y;
    chunk_storage.num_chunks_z = num_chunks_z;

    for x in 0..num_chunks_x {
        for y in 0..num_chunks_y {
            for z in 0..num_chunks_z {
                let chunk_position = Vec3::new(
                    (x * chunk_size.x) as f32,
                    (y * chunk_size.y) as f32,
                    (z * chunk_size.z) as f32,
                );
                chunk_storage.chunks.push(Chunk {
                    world_position: chunk_position,
                    is_visible: false,
                    rendered_cubes: Vec::new(),
                });
            }
        }
    }
}

fn create_heightmap(world_size: Res<WorldSize>, mut chunk_storage: ResMut<ChunkStorage>) {
    let mut hmap = Heightmap::new();
    chunk_storage.hmap = hmap.generate_heightmap(world_size.x, world_size.y);
}

pub struct ChunkPlugin;
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WorldSize {
            x: 10000,
            y: 10,
            z: 10000,
        });

        app.insert_resource(ChunkSize { x: 8, y: 8, z: 8 });

        app.insert_resource(ChunkRenderRadius { radius: 30. });

        app.insert_resource(ChunkStorage {
            chunks: Vec::new(),
            num_chunks_x: 0,
            num_chunks_y: 0,
            num_chunks_z: 0,
            hmap: Vec::new(),
        });

        // app.add_systems(Startup, create_heightmap);
        app.add_systems(Startup, create_chunks);
    }
}

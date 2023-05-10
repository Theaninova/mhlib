use godot::engine::global::Error;
use godot::engine::utilities::{clampi, printerr};
use godot::engine::{load, PackedScene};
use godot::engine::{ImageTexture, TileSet};
use godot::engine::{TileMap, TileSetAtlasSource};
use godot::prelude::*;
use godot::prelude::{Gd, PackedByteArray, Share, ToVariant};
use springylib::media::level::LevelLayer;

pub fn create_tile_map(layer: LevelLayer, level_id: u32) -> Gd<PackedScene> {
    let mut tile_set = TileSet::new();
    tile_set.set_tile_size(Vector2i { x: 32, y: 32 });
    tile_set.add_physics_layer(0);
    let mut map = TileMap::new_alloc();
    map.set_tileset(tile_set.share());
    map.set_quadrant_size(32);

    for x in 0..layer.width {
        for y in 0..layer.height {
            let tile = &layer.tiles[(y * layer.width + x) as usize];
            if tile.id == 0 {
                continue;
            }
            if !tile_set.has_source(tile.id as i64) {
                let atlas_id = tile.id as u32 + 1;
                let atlas = load_atlas(1, atlas_id, layer.tile_count);
                tile_set.add_source(atlas.share().upcast(), tile.id as i64);
                add_collision(atlas, level_id, atlas_id);
            }
            map.set_cell(
                0,
                Vector2i {
                    x: x as i32,
                    y: y as i32,
                },
                tile.id as i64,
                Vector2i {
                    x: clampi(tile.index as i64 % 16, 0, 15) as i32,
                    y: clampi(tile.index as i64 / 16, 0, 15) as i32,
                },
                0,
            );
        }
    }

    let mut scene = PackedScene::new();
    let error = scene.pack(map.upcast());
    match error {
        Error::OK => (),
        e => printerr(e.to_variant(), &[]),
    }
    scene
}

#[derive(GodotClass)]
#[class(base=Resource, init)]
pub struct TileCollision {
    #[export]
    pub collision: PackedByteArray,
}

#[godot_api]
impl TileCollision {}

fn add_collision(atlas: Gd<TileSetAtlasSource>, level_id: u32, atlas_id: u32) {
    let tile_collision: Gd<TileCollision> = load(format!(
        "datafile://data/level{:02}/tile_collision_{:02}.txt",
        level_id, atlas_id
    ));
    let width = atlas.get_atlas_grid_size().x;
    let height = atlas.get_atlas_grid_size().y;

    let tile_width = atlas.get_texture_region_size().x as f32 / 2.0;
    let tile_height = atlas.get_texture_region_size().y as f32 / 2.0;
    let collision = &[
        Vector2 {
            x: -tile_width,
            y: -tile_height,
        },
        Vector2 {
            x: -tile_width,
            y: tile_height,
        },
        Vector2 {
            x: tile_width,
            y: tile_height,
        },
        Vector2 {
            x: tile_width,
            y: -tile_height,
        },
    ];

    for x in 0..width {
        for y in 0..height {
            let collision_data = tile_collision
                .bind()
                .collision
                .get((y * width + x) as usize);
            let mut data = atlas.get_tile_data(Vector2i { x, y }, 0).unwrap();
            if collision_data & 0x1 != 0 {
                data.add_collision_polygon(0);
                data.set_collision_polygon_points(0, 0, PackedVector2Array::from(collision));
            } else if collision_data & 0xfe != 0 {
                printerr(
                    format!("Missing collision info for {}", collision_data).to_variant(),
                    &[],
                );
            }
        }
    }
}

fn load_atlas(set_id: u32, atlas_id: u32, tile_count: u32) -> Gd<TileSetAtlasSource> {
    let mut atlas = TileSetAtlasSource::new();
    let tex: Gd<ImageTexture> = load(format!(
        "datafile://data/set{}/sprites/tiles_{:02}.bmp",
        set_id, atlas_id,
    ));
    let region_size = (tile_count as f32).sqrt();
    debug_assert_eq!(tex.get_width(), tex.get_height());
    debug_assert_eq!(region_size, region_size.trunc());

    let tile_size = (tex.get_width() / region_size as i64) as i32;

    atlas.set_texture(tex.upcast());
    atlas.set_texture_region_size(Vector2i {
        x: tile_size,
        y: tile_size,
    });

    for x in 0..region_size as i32 {
        for y in 0..region_size as i32 {
            atlas.create_tile(Vector2i { x, y }, Vector2i { x: 1, y: 1 });
        }
    }

    atlas
}

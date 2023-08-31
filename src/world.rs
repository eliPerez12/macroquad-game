use crate::{assets::Assets, maps::TILE_COLLIDER_LOOKUP, player::Player};
use macroquad::prelude::*;
use std::collections::HashSet;

pub const LINE_LENGTH: f32 = 20.0 * 8.0;
pub const ANGLE_PERIPHERAL_FACTOR: f32 = 7.2 / 8.0;

#[cfg(debug_assertions)]
pub const RAY_AMOUNT: f32 = 7.5;
#[cfg(not(debug_assertions))]
pub const RAY_AMOUNT: f32 = 15.0;

pub struct TileMap {
    pub data: Vec<u8>,
    pub width: u16,
    pub height: u16,
}

pub fn draw_world(tiles: &TileMap, assets: &Assets, player: &Player) {
    // Calculate tiles that are visible to the rays
    let angles = player.get_player_rays(
        std::f32::consts::PI * ANGLE_PERIPHERAL_FACTOR,
        LINE_LENGTH * ANGLE_PERIPHERAL_FACTOR,
    );
    let visible_tiles = find_tiles(
        angles,
        LINE_LENGTH / 8.0 * ANGLE_PERIPHERAL_FACTOR,
        player.pos,
        tiles,
    );

    // Render
    draw_tiles(tiles, assets, visible_tiles);
}

fn draw_tiles(tiles: &TileMap, assets: &Assets, visible_tiles: HashSet<(u16, u16)>) {
    const FIT_OFFSET: f32 = 0.25;
    const NOT_VISIBLE_TILE_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.0);

    for (tiles_index, tile) in tiles.data.iter().enumerate() {
        let grid_x = tiles_index as u16 % tiles.width;
        let grid_y = tiles_index as u16 / tiles.width;
        let is_collider = TILE_COLLIDER_LOOKUP
            [{ tiles.data[{ grid_x + grid_y * tiles.width } as usize] - 1 } as usize];
        let color = match visible_tiles.contains(&(grid_x, grid_y)) || is_collider {
            true => WHITE,
            false => NOT_VISIBLE_TILE_COLOR,
        };
        draw_texture_ex(
            &assets.get_texture("tiles.png"),
            grid_x as f32 * 8.0,
            grid_y as f32 * 8.0,
            color, // Make into shadow render later
            DrawTextureParams {
                source: Some(Rect::new(
                    ((tile - 1) % 8) as f32 * 8.0 + FIT_OFFSET / 2.0,
                    ((tile - 1) / 8) as f32 * 8.0 + FIT_OFFSET / 2.0,
                    8.0 - FIT_OFFSET,
                    8.0 - FIT_OFFSET,
                )),
                dest_size: Some(Vec2::new(8.0, 8.0)),
                ..Default::default()
            },
        );
    }
}

fn find_tiles(
    angles: Vec<f32>,
    length: f32,
    origin: Vec2,
    tile_map: &TileMap,
) -> HashSet<(u16, u16)> {
    let mut tiles = HashSet::new();

    for angle in angles {
        let (mut x, mut y) = (origin.x / 8.0, origin.y / 8.0);
        let dx = angle.cos();
        let dy = angle.sin();

        let delta_dist_x = (1.0 / dx).abs();
        let delta_dist_y = (1.0 / dy).abs();

        let mut step_x = 1;
        let mut step_y = 1;
        let mut side_dist_x = (x.ceil() - x) * delta_dist_x;
        let mut side_dist_y = (y.ceil() - y) * delta_dist_y;

        if dx < 0.0 {
            step_x = -1;
            side_dist_x = (x - x.floor()) * delta_dist_x;
        }

        if dy < 0.0 {
            step_y = -1;
            side_dist_y = (y - y.floor()) * delta_dist_y;
        }

        let mut reached_length = 0.0;
        let mut keep_casting_ray = true;

        // Keep shooting ray until it reaches a collider or the end of the length
        while reached_length < length && keep_casting_ray {
            if x < 0.0 || y < 0.0 {
                keep_casting_ray = false;
                continue;
            };

            let tile_x = x.floor() as u16;
            let tile_y = y.floor() as u16;

            if tile_x < tile_map.width && tile_y < tile_map.height {
                let tile = tile_map.data[(tile_x + tile_y * tile_map.width) as usize];
                if TILE_COLLIDER_LOOKUP[(tile - 1) as usize] == true {
                    keep_casting_ray = false;
                    continue;
                }
                tiles.insert((tile_x, tile_y));
            }

            // Move to the next cell
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                x += step_x as f32;
                reached_length = side_dist_x;
            } else {
                side_dist_y += delta_dist_y;
                y += step_y as f32;
                reached_length = side_dist_y;
            }
        }
    }
    tiles
}

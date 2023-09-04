use crate::{
    assets::Assets, camera::GameCamera, maps::TILE_COLLIDER_LOOKUP, player::Player,
    utils::draw_rect,
};
use macroquad::prelude::*;
use std::collections::HashSet;

pub const LINE_LENGTH: f32 = 23.0 * 8.0;
pub const ANGLE_PERIPHERAL_FACTOR: f32 = 8.0 / 8.0;

#[cfg(debug_assertions)]
pub const RAY_AMOUNT: f32 = 2.0;
#[cfg(not(debug_assertions))]
pub const RAY_AMOUNT: f32 = 8.0;

pub struct TileMap {
    pub data: Vec<u32>,
    pub width: u16,
    pub height: u16,
}

impl TileMap {
    pub fn rect_collides_with_tile(&self, rect: Rect) -> bool {
        for index in 0..(self.width * self.height) {
            let tile = self.get_tile(index as u16 % self.width, index as u16 / self.width).unwrap().0;

            if !TILE_COLLIDER_LOOKUP[(tile- 1) as usize] {
                continue;
            }
            let tile_grid_x = index as u16 % self.width;
            let tile_grid_y = index as u16 / self.width;
            let tile_rect = Rect::new(tile_grid_x as f32 * 8.0, tile_grid_y as f32 * 8.0, 8.0, 8.0);

            if rect.intersect(tile_rect).is_some() {
                return true;
            }
        }
        false
    }
    pub fn point_collides_with_tile(&self, point: Vec2) -> bool {
        for index in 0..(self.width * self.height) {
            let tile = self.get_tile(index as u16 % self.width, index as u16 / self.width).unwrap().0;
            if !TILE_COLLIDER_LOOKUP[(tile - 1) as usize] {
                continue;
            }
            let tile_grid_x = index as u16 % self.width;
            let tile_grid_y = index as u16 / self.width;
            let tile_rect = Rect::new(tile_grid_x as f32 * 8.0, tile_grid_y as f32 * 8.0, 8.0, 8.0);

            if tile_rect.contains(point) {
                return true;
            }
        }
        false
    }

    // Returns (original_tile, flip_x, flip_y)
    fn get_tile(&self, grid_x: u16, grid_y: u16) -> Option<(u32, bool, bool)> {
        if let Some(tile) = self.data.get((grid_x + grid_y * self.width) as usize) {
            Some((
                tile & 0x1FFFFFFF,        // Get all bits except top three, which returns the original tile
                (tile & 0x80000000) != 0, // Get most significant bit for flip_x
                (tile & 0x40000000) != 0, // Get most lease bit for flip_y
            ))
        } else {
            None
        }
    }
}

fn find_tiles(
    angles: Vec<f32>,
    length: f32,
    origin: Vec2,
    tile_map: &TileMap,
    _camera: &GameCamera,
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
        // Keep shooting ray until it reaches a collider or the end of the length
        while reached_length < length {
            if x < 0.0
                || y < 0.0
                || x > tile_map.width as f32 * 8.0
                || y > tile_map.height as f32 * 8.0
            {
                break;
            };

            let tile_x = x.floor() as u16;
            let tile_y = y.floor() as u16;

            if tile_x < tile_map.width && tile_y < tile_map.height {
                let tile = tile_map.data[(tile_x + tile_y * tile_map.width) as usize];
                tiles.insert((tile_x, tile_y));
                if TILE_COLLIDER_LOOKUP[((tile & 0x3FFFFFFF) - 1) as usize] {
                    break;
                }
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

pub fn draw_world(tiles: &TileMap, assets: &Assets, player: &Player, camera: &GameCamera) {
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
        camera,
    );

    // Render
    draw_tiles(tiles, assets, visible_tiles, camera);
}

fn draw_tiles(
    world: &TileMap,
    assets: &Assets,
    visible_to_player: HashSet<(u16, u16)>,
    camera: &GameCamera,
) {
    const FIT_OFFSET: f32 = 0.25;
    const VISIBLE_COLOR: Color = Color::new(1.0, 1.0, 1.0, 1.0);
    const NOT_VISIBLE_TILE_COLOR: Color = Color::new(0.85, 0.85, 0.85, 1.0);

    let visible_to_camera = camera.get_visible_tiles(&world);

    for (grid_x, grid_y) in visible_to_camera.iter() {
        let (tile, flip_x, flip_y) = match world.get_tile(*grid_x, *grid_y) {
            Some(tile) => tile,
            None => continue,
        };
        let color = match visible_to_player.contains(&(*grid_x, *grid_y)) {
            true => VISIBLE_COLOR,
            false => NOT_VISIBLE_TILE_COLOR,
        };

        draw_texture_ex(
            &assets.get_texture("tiles.png"),
            *grid_x as f32 * 8.0,
            *grid_y as f32 * 8.0,
            color, // Make into shadow render later
            DrawTextureParams {
                source: Some(Rect::new(
                    ((tile - 1) % 8) as f32 * 8.0 + FIT_OFFSET / 2.0,
                    ((tile - 1) / 8) as f32 * 8.0 + FIT_OFFSET / 2.0,
                    8.0 - FIT_OFFSET,
                    8.0 - FIT_OFFSET,
                )),
                dest_size: Some(Vec2::new(8.0, 8.0)),
                flip_x,
                flip_y,

                ..Default::default()
            },
        );
    }
}

pub fn draw_collidables(world: &TileMap, camera: &GameCamera) {
    for (grid_x, grid_y) in camera.get_visible_tiles(world) {
        let tile = world.get_tile(grid_x, grid_y).unwrap().0;
        if TILE_COLLIDER_LOOKUP[(tile - 1) as usize] {
            draw_rect(
                Rect::new(grid_x as f32 * 8.0, grid_y as f32 * 8.0, 8.0, 8.0),
                Color::new(1.0, 0.0, 0.3, 0.75),
            )
        }
    }
}

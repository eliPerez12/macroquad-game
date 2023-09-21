use std::collections::HashSet;

use macroquad::prelude::*;
use crate::{maps::TILE_COLLIDER_LOOKUP, world::{ANGLE_PERIPHERAL_FACTOR, LINE_LENGTH}, player::Player, camera::GameCamera, assets::Assets, utils::draw_rect};

pub struct TileMap {
    pub data: Vec<u32>,
    pub width: u16,
    pub height: u16,
}

impl TileMap {
    pub fn rect_collides_with_tile(&self, rect: Rect) -> bool {
        for index in 0..(self.width * self.height) {
            let tile = self
                .get_tile(index % self.width, index / self.width)
                .unwrap()
                .0;

            if !TILE_COLLIDER_LOOKUP[(tile - 1) as usize] {
                continue;
            }
            let tile_grid_x = index % self.width;
            let tile_grid_y = index / self.width;
            let tile_rect = Rect::new(tile_grid_x as f32 * 8.0, tile_grid_y as f32 * 8.0, 8.0, 8.0);

            if rect.intersect(tile_rect).is_some() {
                return true;
            }
        }
        false
    }

    pub fn point_collides_with_tile(&self, point: Vec2) -> bool {
        for index in 0..(self.width * self.height) {
            let tile = self
                .get_tile(index % self.width, index / self.width)
                .unwrap()
                .0;
            if !TILE_COLLIDER_LOOKUP[(tile - 1) as usize] {
                continue;
            }
            let tile_grid_x = index % self.width;
            let tile_grid_y = index / self.width;
            let tile_rect = Rect::new(tile_grid_x as f32 * 8.0, tile_grid_y as f32 * 8.0, 8.0, 8.0);

            if tile_rect.contains(point) {
                return true;
            }
        }
        false
    }

    // Returns (tile_id, flip_x, flip_y, rotate)
    fn get_tile(&self, grid_x: u16, grid_y: u16) -> Option<(u32, bool, bool, bool)> {
        self.data
            .get((grid_x + grid_y * self.width) as usize)
            .map(|tile| {
                (
                    tile & 0x1FFFFFFF, // Get all bits except top three, which returns the tile id
                    (tile & 0x80000000) != 0, // Get most significant bit for flip_x
                    (tile & 0x40000000) != 0, // Get second bit bit for flip_y
                    (tile & 0x20000000) != 0, // Get third bit for rotate
                )
            })
    }

    pub fn find_tiles(&self, angles: Vec<f32>, length: f32, origin: Vec2) -> HashSet<(u16, u16)> {
        let mut tiles = HashSet::with_capacity(700);
        for angle in angles {
            tiles.extend(self.find_tiles_for_ray(angle, origin, length));
        }
        tiles
    }

    fn find_tiles_for_ray(&self, angle: f32, origin: Vec2, length: f32) -> Vec<(u16, u16)> {
        let mut tiles = Vec::with_capacity(30);
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

        while reached_length < length {
            if x < 0.0 || y < 0.0 || x > self.width as f32 * 8.0 || y > self.height as f32 * 8.0 {
                break;
            }

            let tile_x = x as u16;
            let tile_y = y as u16;

            if tile_x < self.width && tile_y < self.height {
                let tile_id = self.data[(tile_x + tile_y * self.width) as usize] & 0x1FFFFFFF;
                if TILE_COLLIDER_LOOKUP[((tile_id) - 1) as usize] {
                    break;
                }
                tiles.push((tile_x, tile_y));
            }

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
        tiles

    }

    pub fn draw(&self, assets: &Assets, player: &Player, camera: &GameCamera) {
        // Calculate tiles that are visible to the rays
        let angles = player.get_player_rays(
            std::f32::consts::PI * ANGLE_PERIPHERAL_FACTOR,
            LINE_LENGTH * ANGLE_PERIPHERAL_FACTOR,
        );
        let visible_tiles = self.find_tiles(
            angles,
            LINE_LENGTH / 8.0 * ANGLE_PERIPHERAL_FACTOR,
            player.pos,
        );

        // Render
        self.draw_tiles(assets, visible_tiles, camera);
    }

    fn draw_tiles(
        &self,
        assets: &Assets,
        visible_to_player: HashSet<(u16, u16)>,
        camera: &GameCamera,
    ) {
        const FIT_OFFSET: f32 = 0.25;
        const VISIBLE_COLOR: Color = Color::new(1.0, 1.0, 1.0, 1.0);
        const NOT_VISIBLE_TILE_COLOR: Color = Color::new(0.83, 0.83, 0.83, 1.0);

        let visible_to_camera = camera.get_visible_tiles(self);

        for (grid_x, grid_y) in visible_to_camera.iter() {
            let (tile_id, flip_x, flip_y, rotate) = match self.get_tile(*grid_x, *grid_y) {
                Some(tile) => tile,
                None => continue,
            };
            let color = match visible_to_player.contains(&(*grid_x, *grid_y)) {
                true => VISIBLE_COLOR,
                false => NOT_VISIBLE_TILE_COLOR,
            };

            let mut draw_params = DrawTextureParams {
                source: Some(Rect::new(
                    ((tile_id - 1) % 8) as f32 * 8.0 + FIT_OFFSET / 2.0,
                    ((tile_id - 1) / 8) as f32 * 8.0 + FIT_OFFSET / 2.0,
                    8.0 - FIT_OFFSET,
                    8.0 - FIT_OFFSET,
                )),
                dest_size: Some(Vec2::new(8.0, 8.0)),
                flip_x,
                flip_y,
                rotation: if rotate {
                    std::f32::consts::PI / 2.0
                } else {
                    0.0
                },
                ..Default::default()
            };

            // Correct for rotate
            if rotate {
                draw_params.flip_y = !draw_params.flip_y;
                if flip_x {
                    draw_params.flip_x = !draw_params.flip_x;
                }
            }

            draw_texture_ex(
                &assets.get_texture("tiles.png"),
                *grid_x as f32 * 8.0,
                *grid_y as f32 * 8.0,
                color, // Make into shadow render later
                draw_params,
            );
        }
    }

    pub fn draw_collidables(&self, camera: &GameCamera) {
        for (grid_x, grid_y) in camera.get_visible_tiles(self) {
            let tile = self.get_tile(grid_x, grid_y).unwrap().0;
            if TILE_COLLIDER_LOOKUP[(tile - 1) as usize] {
                draw_rect(
                    Rect::new(grid_x as f32 * 8.0, grid_y as f32 * 8.0, 8.0, 8.0),
                    Color::new(1.0, 0.0, 0.3, 0.75),
                )
            }
        }
    }
}

use crate::{
    assets::Assets,
    camera::GameCamera,
    maps::TILE_COLLIDER_LOOKUP,
    player::Player,
    utils::draw_rect,
    world::{ANGLE_PERIPHERAL_FACTOR, LINE_LENGTH},
};
use macroquad::prelude::*;
use std::collections::HashSet;

pub struct LineSegment {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

pub struct TileMap {
    pub data: Vec<u32>,
    pub collidables: HashSet<(u16, u16)>,
    pub width: u16,
    pub height: u16,
}

impl LineSegment {
    // Check if two line segments intersect
    fn line_segments_intersect(&self, b: &LineSegment) -> Option<Vec2> {
        let a = self;
        let denominator = (a.y2 - a.y1) * (b.x2 - b.x1) - (a.x2 - a.x1) * (b.y2 - b.y1);
        if denominator == 0.0 {
            return None;
        }

        let ua = ((a.x2 - a.x1) * (b.y1 - a.y1) - (a.y2 - a.y1) * (b.x1 - a.x1)) / denominator;
        let ub = ((b.x2 - b.x1) * (b.y1 - a.y1) - (b.y2 - b.y1) * (b.x1 - a.x1)) / denominator;

        if (0.0..=1.0).contains(&ua) && (0.0..=1.0).contains(&ub) {
            let x = a.x1 + ua * (a.x2 - a.x1);
            let y = a.y1 + ua * (a.y2 - a.y1);
            return Some((x, y).into());
        }
        None
    }

    // returns lines from a rectangle (left, right, top, bottom)
    pub fn from_rect(rect: &Rect) -> (LineSegment, LineSegment, LineSegment, LineSegment) {
        (
            LineSegment {
                x1: rect.x,
                y1: rect.y,
                x2: rect.x,
                y2: rect.y + rect.h,
            },
            LineSegment {
                x1: rect.x + rect.w,
                y1: rect.y,
                x2: rect.x + rect.w,
                y2: rect.y + rect.h,
            },
            LineSegment {
                x1: rect.x,
                y1: rect.y,
                x2: rect.x + rect.w,
                y2: rect.y,
            },
            LineSegment {
                x1: rect.x,
                y1: rect.y + rect.h,
                x2: rect.x + rect.w,
                y2: rect.y + rect.h,
            },
        )
    }

    // Check if a line segment intersects with a rectangle
    pub fn line_intersects_rect(&self, rect: Rect) -> Option<Vec2> {
        // Edges of the rectangle
        let (left, right, top, bottom) = LineSegment::from_rect(&rect);


        if let Some(intersect) = self.line_segments_intersect(&left) {
            return Some(intersect);
        }
        if let Some(intersect) = self.line_segments_intersect(&right) {
            return Some(intersect);
        } 
        if let Some(intersect) = self.line_segments_intersect(&top) {
            return Some(intersect);
        } 
        if let Some(intersect) = self.line_segments_intersect(&bottom) {
            return Some(intersect);
        }
        None
    }

    pub fn draw(&self, color: Color) {
        draw_line(self.x1, self.y1, self.x2, self.y2, 0.28, color);
    }
}

impl TileMap {
    pub fn rect_collides_with_tile(&self, rect: Rect) -> bool {
        for (grid_x, grid_y) in self.collidables.iter() {
            if let Some(tile) = self.get_tile(*grid_x, *grid_y) {
                if TILE_COLLIDER_LOOKUP
                    .get((tile.0 - 1) as usize)
                    .unwrap_or(&false)
                    == &false
                {
                    continue;
                }

                let tile_rect = Rect::new(*grid_x as f32 * 8.0, *grid_y as f32 * 8.0, 8.0, 8.0);

                if rect.intersect(tile_rect).is_some() {
                    return true;
                }
            }
        }
        false
    }

    pub fn line_collides_with_tile(&self, line: &LineSegment) -> Option<Vec2> {
        for (grid_x, grid_y) in self.collidables.iter() {
            if let Some(tile) = self.get_tile(*grid_x, *grid_y) {
                if let Some(is_collider) = TILE_COLLIDER_LOOKUP.get(tile.0 as usize - 1) {
                    let intersects = line.line_intersects_rect(Rect {
                        x: *grid_x as f32 * 8.0,
                        y: *grid_y as f32 * 8.0,
                        w: 8.0,
                        h: 8.0,
                    });
                    if *is_collider {
                        if let Some(intersects) = intersects {
                            return Some(intersects);
                        }
                    }
                }
            }
        }
        None
    }

    // Returns (tile_id, flip_x, flip_y, rotate)
    pub fn get_tile(&self, grid_x: u16, grid_y: u16) -> Option<(u32, bool, bool, bool)> {
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
            let tile_x = x as u16;
            let tile_y = y as u16;
            let tile_id = self.data[(tile_x + tile_y * self.width) as usize] & 0x1FFFFFFF;
            if TILE_COLLIDER_LOOKUP[((tile_id) - 1) as usize] {
                break;
            }
            tiles.push((tile_x, tile_y));

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
            let tile = self.get_tile(grid_x, grid_y);
            if let Some(tile) = tile {
                if TILE_COLLIDER_LOOKUP[tile.0 as usize - 1] {
                    draw_rect(
                        Rect::new(grid_x as f32 * 8.0, grid_y as f32 * 8.0, 8.0, 8.0),
                        Color::new(1.0, 0.0, 0.3, 0.75),
                    );
                    let rect = Rect {
                        x: grid_x as f32 * 8.0,
                        y: grid_y as f32 * 8.0,
                        w: 8.0,
                        h: 8.0,
                    };

                    let (left, right, top, bottom) = LineSegment::from_rect(&rect);
                    let color = ORANGE;
                    left.draw(color);
                    right.draw(color);
                    top.draw(color);
                    bottom.draw(color);
                }
            }
        }
    }
}

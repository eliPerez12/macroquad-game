use std::collections::HashSet;
use macroquad::prelude::*;
use crate::{assets::Assets, player::Player};

pub struct TileMap {
    pub data: Vec<u8>,
    pub width: u16,
    pub height: u16,
}

fn draw_tiles(tiles: &TileMap, assets: &Assets, visible_tiles: HashSet<(u16, u16)>) {
    for (tiles_index, tile) in tiles.data.iter().enumerate() {
        let fit_offset = 0.0675;
        let grid_x = tiles_index as u16 % tiles.width;
        let grid_y = tiles_index as u16 / tiles.width;
        let color = match visible_tiles.contains(&(grid_x, grid_y)) {
            true => Color::from_rgba(255, 255, 255, 255),
            false => Color::from_rgba(200, 200, 200, 255),
        };
        draw_texture_ex(
            assets.get_texture("tiles.png"),
            grid_x as f32 * 8.0,
            grid_y as f32 * 8.0,
            color, // Make into shadow render later
            DrawTextureParams {
                source: Some(Rect::new(
                    ((tile - 1) % 8) as f32 * 8.0 + fit_offset/2.0,
                    ((tile - 1) / 8) as f32 * 8.0 + fit_offset/2.0,
                    8.0 - fit_offset,
                    8.0 - fit_offset)
                ),
                dest_size: Some(Vec2::new(8.0, 8.0)),
                ..Default::default()
            }
        );
    }
}

// Get tiles visible to a ray
fn find_tiles(origin: Vec2, angle: f32, length: f32, definition: f32) -> HashSet<(u16, u16)> {
    let mut tiles = HashSet::new();
    let adjusted_angle = angle + std::f32::consts::FRAC_PI_2;
    let (mut x, mut y) = (origin.x / 8.0, origin.y / 8.0);
    let (dx, dy) = (adjusted_angle.cos() / definition, adjusted_angle.sin() / definition);
    let loop_count = (length * definition) as i32;

    for _ in 0..loop_count {
        let tile_x = x.floor() as u16;
        let tile_y = y.floor() as u16;
        tiles.insert((tile_x, tile_y));
        x += dx;
        y += dy;
    }
    tiles
}

fn combine_hashsets(hashsets: Vec<HashSet<(u16, u16)>>) -> HashSet<(u16, u16)> {
    let mut combined = HashSet::new();
    for set in hashsets {
        combined.extend(set);
    }
    combined
}

pub fn draw_world(tiles: &TileMap, assets: &Assets, player: &Player) {
    let line_length = 10.0 * 8.0;
    let visible_angle = std::f32::consts::PI;
    let ray_amount = {
        let amount = (visible_angle / 0.05) as i32;
        if amount % 2 == 0 { amount + 1 } else { amount }
    };
    let angle_increment = visible_angle / ray_amount as f32;

    // Get angles for rays that are evenly divided in the field of view
    let angles: Vec<f32> = (0..ray_amount)
        .map(|ray| {
            player.angle
                + angle_increment * ray as f32
                - visible_angle / 2.0
                + angle_increment / 2.0
        })
        .collect();
    
    // Calculate tiles that are visible to the rays
    let definition = 2.5;
    let visible_tiles = combine_hashsets(
        angles
            .iter()
            .map(|&angle| find_tiles(player.pos, angle, line_length / 8.0, definition))
            .collect(),
    );

    // Render 
    draw_tiles(tiles, assets, visible_tiles);
    for angle in &angles {
        draw_line(
            player.pos.x, 
            player.pos.y,
            player.pos.x + (angle + std::f32::consts::FRAC_PI_2).cos() * line_length,
            player.pos.y + (angle + std::f32::consts::FRAC_PI_2).sin() * line_length,
            1.0,
            RED,
        );
    }
}
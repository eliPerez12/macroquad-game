use std::collections::HashSet;
use macroquad::prelude::*;
use crate::{assets::Assets, player::Player};

#[derive(Eq, PartialEq, Hash, Clone)]
enum Visibility {
    NotVisible,
    Direct,
    Peripheral,
}

pub struct TileMap {
    pub data: Vec<u8>,
    pub width: u16,
    pub height: u16,
}

pub fn draw_world(tiles: &TileMap, assets: &Assets, player: &Player) {
    
    // Calculate tiles that are visible to the rays
    let definition = 2.5;
    let line_length = 20.0 * 8.0;
    let angles = player.get_player_rays(std::f32::consts::PI, line_length);
    let visible_tiles = combine_hashsets(
        angles
            .iter()
            .map(|&angle| find_visible_tiles(player.pos, angle, line_length / 8.0, definition))
            .collect(),
    );

    // Render 
    draw_tiles(tiles, assets, visible_tiles);
}

fn hashet_contains(grid_pos: (u16, u16), hashset: &HashSet<(u16, u16, Visibility)>) -> Visibility {
    if hashset.contains(&(grid_pos.0, grid_pos.1, Visibility::Direct)) {
        return Visibility::Direct;
    };
    if hashset.contains(&(grid_pos.0, grid_pos.1, Visibility::Peripheral)) {
        return Visibility::Peripheral;
    };
    Visibility::NotVisible
}

fn draw_tiles(tiles: &TileMap, assets: &Assets,visible_tiles: HashSet<(u16, u16, Visibility)>) {
    for (tiles_index, tile) in tiles.data.iter().enumerate() {
        let fit_offset = 0.25;
        let grid_x = tiles_index as u16 % tiles.width;
        let grid_y = tiles_index as u16 / tiles.width;
        let color = match hashet_contains((grid_x, grid_y), &visible_tiles) {
            Visibility::Direct => WHITE,
            Visibility::Peripheral => Color::new(0.9, 0.9, 0.9, 1.0),
            Visibility::NotVisible => Color::new(0.8, 0.8, 0.8, 1.0)
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

fn find_tiles(
    tiles: &mut HashSet<(u16, u16, Visibility)>,
    angle: f32,
    length: f32,
    definition: f32, 
    origin: Vec2,
    visibility: Visibility,
) {
    let angle = angle + std::f32::consts::FRAC_PI_2;
    
    let (mut x, mut y) = (origin.x / 8.0, origin.y / 8.0);
    let (dx, dy) = (angle.cos() / definition, angle.sin() / definition);
    let loop_count = (length * definition) as i32;

    // Find tiles that are in direct view
    for _ in 0..loop_count {
        let tile_x = {
            let x = x.floor();
            if x < 0.0 {continue}
            else {x as u16}
        };
        let tile_y = {
            let y = y.floor();
            if y < 0.0 {continue}
            else {y as u16}
        };
        tiles.insert((tile_x, tile_y, visibility.clone()));
        x += dx;
        y += dy;
    }
}


// Get tiles visible to a ray
fn find_visible_tiles(origin: Vec2, angle: f32, length: f32, definition: f32) -> HashSet<(u16, u16, Visibility)> {
    let mut tiles = HashSet::new();
    find_tiles(&mut tiles, angle, length, definition, origin, Visibility::Peripheral);
    find_tiles(&mut tiles, angle * 7.0/8.0 , length * 7.0/8.0, definition, origin, Visibility::Direct);
    tiles
}

fn combine_hashsets(hashsets: Vec<HashSet<(u16, u16, Visibility)>>) -> HashSet<(u16, u16, Visibility)> {
    let mut combined = HashSet::new();
    for set in hashsets {
        combined.extend(set);
    }
    combined
}

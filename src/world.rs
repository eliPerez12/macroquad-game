use std::collections::HashSet;
use macroquad::prelude::*;
use crate::{assets::Assets, player::Player};

const RAY_DEFINITION: f32 = 2.1;
const LINE_LENGTH: f32 = 20.0 * 8.0;
const ANGLE_PERIPHERAL_FACTOR: f32 = 6.6/8.0;


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


impl TileMap {
    fn get_tile(&self, grid_pos: (u16, u16)) -> Option<&u8>{
        self.data.get((grid_pos.0 + grid_pos.1 * self.width) as usize)
    }
}

pub fn draw_world(tiles: &TileMap, assets: &Assets, player: &Player) {
    
    // Calculate tiles that are visible to the rays
    let direct_angles = player.get_player_rays(std::f32::consts::PI * ANGLE_PERIPHERAL_FACTOR, LINE_LENGTH * ANGLE_PERIPHERAL_FACTOR);
    let peripheral_angles = player.get_player_rays(std::f32::consts::PI, LINE_LENGTH);

    let visible_tiles = combine_hashsets(vec![combine_hashsets(
        peripheral_angles
            .iter()
            .map(|&angle| find_visible_tiles(
                player.pos,
                angle,
                LINE_LENGTH / 8.0,
                Visibility::Peripheral
            )
        ).collect(),
    ), combine_hashsets(
        direct_angles
            .iter()
            .map(|&angle| find_visible_tiles(
                player.pos,
                angle,
                LINE_LENGTH / 8.0 * ANGLE_PERIPHERAL_FACTOR,
                Visibility::Direct
            )
        ).collect(),
    )]);

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
    const FIT_OFFSET: f32 = 0.25;
    const PERIPHERAL_TILE_COLOR: Color = Color::new(0.9, 0.9, 0.9, 1.0);
    const NOT_VISIBLE_TILE_COLOR: Color = Color::new(0.8, 0.8, 0.8, 1.0);

    for (tiles_index, tile) in tiles.data.iter().enumerate() {
        let grid_x = tiles_index as u16 % tiles.width;
        let grid_y = tiles_index as u16 / tiles.width;
        let color = match hashet_contains((grid_x, grid_y), &visible_tiles) {
            Visibility::Direct => WHITE,
            Visibility::Peripheral => PERIPHERAL_TILE_COLOR,
            Visibility::NotVisible => NOT_VISIBLE_TILE_COLOR,
        };
        draw_texture_ex(
            assets.get_texture("tiles.png"),
            grid_x as f32 * 8.0,
            grid_y as f32 * 8.0,
            color, // Make into shadow render later
            DrawTextureParams {
                source: Some(Rect::new(
                    ((tile - 1) % 8) as f32 * 8.0 + FIT_OFFSET /2.0,
                    ((tile - 1) / 8) as f32 * 8.0 + FIT_OFFSET /2.0,
                    8.0 - FIT_OFFSET,
                    8.0 - FIT_OFFSET)
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
    origin: Vec2,
    visibility: Visibility,
) {
    let angle = angle + std::f32::consts::FRAC_PI_2;
    
    let (mut x, mut y) = (origin.x / 8.0, origin.y / 8.0);
    let (dx, dy) = (angle.cos() / RAY_DEFINITION, angle.sin() / RAY_DEFINITION);
    let loop_count = (length * RAY_DEFINITION) as i32;

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
fn find_visible_tiles(origin: Vec2, angle: f32, length: f32,  visibility: Visibility) -> HashSet<(u16, u16, Visibility)> {
    let mut tiles = HashSet::new();
    find_tiles(&mut tiles, angle , length, origin, visibility);
    tiles
}

fn combine_hashsets(hashsets: Vec<HashSet<(u16, u16, Visibility)>>) -> HashSet<(u16, u16, Visibility)> {
    let mut combined = HashSet::new();
    for set in hashsets {
        combined.extend(set);
    }
    combined
}

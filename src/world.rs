use crate::{
    assets::Assets, camera::GameCamera, player::Player,
    entities::EntityManager, maps, tile_map::TileMap,
};
use macroquad::prelude::*;

pub const LINE_LENGTH: f32 = 23.0 * 8.0;
pub const ANGLE_PERIPHERAL_FACTOR: f32 = 1.0;

#[cfg(debug_assertions)]
pub const RAY_AMOUNT: f32 = 1.8;
#[cfg(not(debug_assertions))]
pub const RAY_AMOUNT: f32 = 8.0;

pub struct World {
    pub tile_map: TileMap,
    pub entities: EntityManager,
}

impl World {
    pub fn new() -> Self {
        World { tile_map: maps::example_world(), entities: EntityManager::new() }
    }

    pub fn update(&mut self, player: &Player, camera: &GameCamera, assets: &Assets) {
        self.entities.handle_shooting(assets, player, camera, &self.tile_map);
        self.entities.update(player, camera);
    }

    pub fn draw(&self, camera: &GameCamera, player: &Player, assets: &Assets) {
        // Draws example world
        self.tile_map.draw(assets, player, camera);

        // Draw entities
        self.entities.draw_entities(assets, player, &self.tile_map);
    }

    pub fn draw_debug(&self, camera: &GameCamera) {
        self.entities.draw_entity_hitboxes();
        self.tile_map.draw_collidables(camera);
    }
}

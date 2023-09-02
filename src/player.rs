use std::f32::consts::FRAC_PI_2;

use crate::{
    camera::GameCamera, items::Item, utils::draw_rect, world::ANGLE_PERIPHERAL_FACTOR,
    world::{LINE_LENGTH, TileMap}, world::RAY_AMOUNT, Assets, maps::TILE_COLLIDER_LOOKUP,
};
use macroquad::prelude::*;

pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub health: f32,
    pub stamina: f32,
    pub angle: f32,

    pub movement_state: PlayerMovementState,
    pub stamina_state: PlayerStaminaState,
    pub clothes: Item::Clothes,
    pub gun: Item::Gun,
}

#[derive(PartialEq, Eq)]
pub enum PlayerMovementState {
    Idle,
    Walking,
    Sprinting,
}

#[derive(PartialEq, Eq)]
pub enum PlayerStaminaState {
    Normal,
    Recovering,
}

// Movement Logic
impl Player {
    const MAX_STAMINA: f32 = 100.0;
    const MIN_STAMINA_FOR_SPRINTING: f32 = 10.0;

    const SPRINTING_VELOCITY: f32 = 0.40;
    const WALKING_VELOCITY: f32 = 0.25;
    const PLAYER_ACC: f32 = 0.1; // Acceleration
    const PLAYER_DEACC: f32 = 0.05; // Deacceleration

    const STAMINA_REGEN: f32 = 0.07;
    const STAMINA_COST: f32 = 0.2;
    const STAMINA_AIMING_COST: f32 = 0.1;

    pub fn new() -> Player {
        Player {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            stamina: Player::MAX_STAMINA,
            movement_state: PlayerMovementState::Idle,
            stamina_state: PlayerStaminaState::Normal,
            angle: 0.0,
            health: 100.0,
            clothes: Item::Clothes::dark_clothes(),
            gun: Item::Gun::sawed_shotgun(),
        }
    }

    // Function to handle player movements
    pub fn handle_player_movements(&mut self, camera: &GameCamera, tile_map: &TileMap) {
        // Update
        self.handle_movement_state();
        self.handle_velocity();
        self.handle_collisions(tile_map);
        self.handle_stamina();
        self.handle_gun_controls();
        self.handle_clothes_controls();

        // Apply
        self.apply_velocity();
        self.update_angle_to_mouse(camera);
    }

    fn handle_gun_controls(&mut self) {
        if is_key_pressed(KeyCode::Key1) {
            self.gun = Item::Gun::sawed_shotgun()
        }
        if is_key_pressed(KeyCode::Key2) {
            self.gun = Item::Gun::sniper()
        }
    }

    fn handle_clothes_controls(&mut self) {
        if is_key_pressed(KeyCode::Key3) {
            self.clothes = Item::Clothes::blue_clothes()
        }
        if is_key_pressed(KeyCode::Key4) {
            self.clothes = Item::Clothes::dark_clothes()
        }
    }

    fn handle_velocity(&mut self) {
        // Determine max velocity based on running state
        let player_max_vel: f32 = {
            if self.movement_state == PlayerMovementState::Sprinting
                && self.stamina_state == PlayerStaminaState::Normal
            {
                Player::SPRINTING_VELOCITY
            } else {
                Player::WALKING_VELOCITY
            }
        };

        // Handle movement inputs
        if is_key_down(KeyCode::W) {
            self.vel.y -= Player::PLAYER_ACC;
        }
        if is_key_down(KeyCode::S) {
            self.vel.y += Player::PLAYER_ACC;
        }
        if is_key_down(KeyCode::A) {
            self.vel.x -= Player::PLAYER_ACC;
        }
        if is_key_down(KeyCode::D) {
            self.vel.x += Player::PLAYER_ACC;
        }

        // Normalize velocity to maintain constant speed
        let magnitude = (self.vel.x.powi(2) + self.vel.y.powi(2)).sqrt();
        if magnitude > player_max_vel {
            self.vel.x = (self.vel.x / magnitude) * player_max_vel;
            self.vel.y = (self.vel.y / magnitude) * player_max_vel;
        }

        // Deacceleration logic when keys are not pressed
        if self.vel.x > 0.0 && !is_key_down(KeyCode::D) {
            self.vel.x = (self.vel.x - Player::PLAYER_DEACC).max(0.0);
        }
        if self.vel.y > 0.0 && !is_key_down(KeyCode::S) {
            self.vel.y = (self.vel.y - Player::PLAYER_DEACC).max(0.0);
        }
        if self.vel.x < 0.0 && !is_key_down(KeyCode::A) {
            self.vel.x = (self.vel.x + Player::PLAYER_DEACC).min(0.0);
        }
        if self.vel.y < 0.0 && !is_key_down(KeyCode::W) {
            self.vel.y = (self.vel.y + Player::PLAYER_DEACC).min(0.0);
        }
    }

    // Handles stamina regeneration, recovery and depletion
    fn handle_stamina(&mut self) {
        // Regular spriting cost
        if self.movement_state == PlayerMovementState::Sprinting
            && self.stamina_state == PlayerStaminaState::Normal
        {
            self.stamina = (self.stamina - Player::STAMINA_COST * get_frame_time() * 60.0).max(0.0); // Deplete stamina for running
        }
        if self.is_aiming() {
            self.stamina =
                (self.stamina - Player::STAMINA_AIMING_COST * get_frame_time() * 60.0).max(0.0);
        }
        // Enter recovering stamina state
        if self.stamina <= 0.0 {
            self.stamina_state = PlayerStaminaState::Recovering
        }
        // Regen stamina
        
        self.stamina = (self.stamina + Player::STAMINA_REGEN * get_frame_time() * 60.0)
            .min(Player::MAX_STAMINA);

        if self.stamina >= Player::MIN_STAMINA_FOR_SPRINTING {
            self.stamina_state = PlayerStaminaState::Normal;
        }
    }

    // Update player movement state based on inputs
    fn handle_movement_state(&mut self) {
        self.movement_state = {
            match (self.is_moving(), is_key_down(KeyCode::LeftShift)) {
                (true, true) => PlayerMovementState::Sprinting,
                (true, false) => PlayerMovementState::Walking,
                _ => PlayerMovementState::Idle,
            }
        }
    }
    
    
    fn apply_velocity(&mut self) {
        self.pos += self.vel* get_frame_time() * 60.0;

    }

    // Function to update the player's angle towards the mouse position
    pub fn update_angle_to_mouse(&mut self, camera: &GameCamera) {
        let mouse_pos: Vec2 = mouse_position().into();
        let screen_center = camera.world_to_screen(self.pos);
        let mouse_dist_center = mouse_pos - screen_center;
        self.angle = f32::atan2(-mouse_dist_center.x, mouse_dist_center.y);
    }

    // Checks if any keys are down that would move the player
    fn is_moving(&self) -> bool {
        if is_key_down(KeyCode::W) {
            return true;
        }
        if is_key_down(KeyCode::S) {
            return true;
        }
        if is_key_down(KeyCode::A) {
            return true;
        }
        if is_key_down(KeyCode::D) {
            return true;
        }
        false
    }

    fn is_aiming(&self) -> bool {
        is_mouse_button_down(MouseButton::Right)
    }

    pub fn get_player_rays(&self, fov: f32, line_length: f32) -> Vec<f32> {
        let ray_amount = {
            let amount = (fov * line_length * RAY_AMOUNT) as i32;
            if amount % 2 == 0 {
                amount + 1
            } else {
                amount
            }
        };
        let angle_increment = fov / ray_amount as f32;

        // Get angles for rays that are evenly divided in the field of view
        let angles: Vec<f32> = (0..ray_amount)
            .map(|ray| {
                self.angle + angle_increment * ray as f32 - fov / 2.0
                    + angle_increment / 2.0
                    + FRAC_PI_2
            })
            .collect();
        angles
    }

    pub fn get_hitbox(&self) -> Rect {
        let rect_size = 4.7;
        Rect {
            x: self.pos.x - rect_size / 2.0,
            y: self.pos.y - rect_size / 2.0,
            w: rect_size,
            h: rect_size,
        }
    }

    fn handle_collisions(&mut self, tile_map: &TileMap) {
        let player_hitbox = self.get_hitbox();


        // X vel
        if tile_map.rect_collides_with_tile(Rect::new(player_hitbox.x + self.vel.x, player_hitbox.y, player_hitbox.w, player_hitbox.h)) {
            self.vel.x = 0.0;
        }
        // Y vel
        if tile_map.rect_collides_with_tile(Rect::new(player_hitbox.x, player_hitbox.y + self.vel.y , player_hitbox.w, player_hitbox.h)) {
            self.vel.y = 0.0;
        }
    }
}

// Drawing logic
impl Player {
    fn draw_on_player(&self, texture: &Texture2D) {
        const CENTER_OFFSET: f32 = 1.0 / 6.0;
        draw_texture_ex(
            texture,
            self.pos.x - (17.0 * 1.3333333) / 2.0 + CENTER_OFFSET,
            self.pos.y - (17.0 * 1.3333333) / 2.0 - CENTER_OFFSET,
            WHITE,
            DrawTextureParams {
                rotation: self.angle,
                pivot: Some(self.pos),
                dest_size: Some(Vec2::new(17.0 * 1.3333333, 17.0 * 1.3333333)),
                ..Default::default()
            },
        );
    }

    pub fn draw(&self, assets: &Assets) {
        const CENTER_OFFSET: f32 = 1.0 / 6.0;

        // Get gun texture
        let gun_name = self.gun.name;
        let gun_texture = match self.is_aiming() {
            true => assets.get_texture(&format!("{gun_name}_aiming.png")),
            false => assets.get_texture(&format!("{gun_name}_idle.png")),
        };

        // Get player texture
        let clothes_name = self.clothes.name;
        let player_texture = match self.is_aiming() {
            true => assets.get_texture(&format!("{clothes_name}_clothes_aiming.png")),
            false => assets.get_texture(&format!("{clothes_name}_clothes_idle.png")),
        };

        // Get backpack texture
        let backpack_texture = assets.get_texture("backpack.png");

        // Draw player shadow
        draw_circle(
            self.pos.x + CENTER_OFFSET + 0.3,
            self.pos.y - CENTER_OFFSET + 0.3,
            3.2,
            Color::from_rgba(0, 0, 0, 70),
        );

        // Draw entire player
        self.draw_on_player(&gun_texture);
        self.draw_on_player(&player_texture);
        self.draw_on_player(&backpack_texture);
    }

    pub fn draw_hitbox(&self) {
        draw_rect(self.get_hitbox(), Color::new(0.5, 1.0, 0.0, 0.8))
    }

    pub fn _draw_debug_rays(&self) {
        for angle in self.get_player_rays(
            std::f32::consts::PI * ANGLE_PERIPHERAL_FACTOR,
            LINE_LENGTH * ANGLE_PERIPHERAL_FACTOR,
        ) {
            draw_line(
                self.pos.x,
                self.pos.y,
                self.pos.x + angle.cos() * LINE_LENGTH * ANGLE_PERIPHERAL_FACTOR,
                self.pos.y + angle.sin() * LINE_LENGTH * ANGLE_PERIPHERAL_FACTOR,
                0.5,
                RED,
            );
        }
    }
}

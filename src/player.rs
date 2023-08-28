use crate::{camera::GameCamera, Assets, utils::draw_rect};
use macroquad::prelude::*;

pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub health: f32,
    pub stamina: f32,
    pub movement_state: PlayerMovementState,
    pub stamina_state: PlayerStaminaState,
    pub clothes: PlayerClothes,
    pub angle: f32,
}

pub enum PlayerClothes {
    Blue,
    Dark,
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
    const MIN_STAMINA_FOR_SPRINTING: f32 = 15.0;

    const SPRINTING_VELOCITY: f32 = 0.40;
    const WALKING_VELOCITY: f32 = 0.28;
    const PLAYER_ACC: f32 = 0.1; // Acceleration
    const PLAYER_DEACC: f32 = 0.05; // Deacceleration

    pub fn new() -> Player {
        Player {
            pos: Vec2::ZERO,
            vel: Vec2::ZERO,
            stamina: Player::MAX_STAMINA,
            movement_state: PlayerMovementState::Idle,
            stamina_state: PlayerStaminaState::Normal,
            angle: 0.0,
            health: 100.0,
            clothes: PlayerClothes::Dark,
        }
    }

    // Function to handle player movements
    pub fn handle_player_movements(&mut self, camera: &GameCamera) {
        self.handle_movement_state();
        self.handle_velocity();
        self.handle_stamina();
        self.apply_velocity();
        self.update_angle_to_mouse(camera);
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
        if self.movement_state == PlayerMovementState::Sprinting && self.stamina <= 0.0 {
            // Enter recovering stamina state
            self.stamina_state = PlayerStaminaState::Recovering
        }
        if self.movement_state == PlayerMovementState::Sprinting
            && self.stamina_state == PlayerStaminaState::Normal
        {
            self.stamina = (self.stamina - 0.2 * get_frame_time() * 60.0).max(0.0);
        } else if self.stamina < Player::MAX_STAMINA && !self.is_aiming() {
            self.stamina = (self.stamina + 0.1 * get_frame_time() * 60.0).min(Player::MAX_STAMINA);
            if self.stamina >= Player::MIN_STAMINA_FOR_SPRINTING {
                self.stamina_state = PlayerStaminaState::Normal;
            }
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
        self.pos += self.vel * get_frame_time() * 60.0;
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

    pub fn get_player_rays(&self, line_length: f32) -> Vec<f32> {
        let visible_angle = std::f32::consts::PI;
        let ray_amount = {
            let amount = (visible_angle / 0.02 * line_length/80.0) as i32;
            if amount % 2 == 0 { amount + 1 } else { amount }
        };
        let angle_increment = visible_angle / ray_amount as f32;
    
        // Get angles for rays that are evenly divided in the field of view
        let angles: Vec<f32> = (0..ray_amount)
            .map(|ray| {
                self.angle
                    + angle_increment * ray as f32
                    - visible_angle / 2.0
                    + angle_increment / 2.0
            })
            .collect();
        angles
    }

    pub fn draw_debug_rays(&self) {
        let line_length = 20.0 * 8.0;
        for angle in self.get_player_rays(line_length) {
            draw_line(
                self.pos.x, 
                self.pos.y,
                self.pos.x + (angle + std::f32::consts::FRAC_PI_2).cos() * line_length,
                self.pos.y + (angle + std::f32::consts::FRAC_PI_2).sin() * line_length,
                0.5,
                RED,
            );
        }
    }

    pub fn get_hitbox(&self) -> Rect {
        let rect_size = 4.7;
        Rect {
            x: self.pos.x - rect_size/2.0,
            y: self.pos.y - rect_size/2.0,
            w: rect_size,
            h: rect_size, 
        }
    }

    pub fn draw_hitbox(&self) {
        draw_rect(self.get_hitbox(), Color::new(1.0, 0.0, 0.0, 0.8))
    }
}

// Drawing logic
impl Player {
    pub fn draw(&self, assets: &Assets) {
        const CENTER_OFFSET: f32 = 1.0 / 6.0;

        // Draw player shadow
        draw_circle(
            self.pos.x + CENTER_OFFSET + 0.3,
            self.pos.y - CENTER_OFFSET + 0.3,
            3.2,
            Color::from_rgba(0, 0, 0, 70),
        );

        let player_texture = match self.is_aiming() {
            false => assets.get_texture(match self.clothes {
                PlayerClothes::Blue => "player_blue.png",
                PlayerClothes::Dark => "player_dark.png",
            }),
            true => assets.get_texture(match self.clothes {
                PlayerClothes::Blue => "player_blue_aiming.png",
                PlayerClothes::Dark => "player_dark_aiming.png",
            }),
        };

        // Draw player
        draw_texture_ex(
            &player_texture,
            self.pos.x - 17.0 / 2.0 + CENTER_OFFSET,
            self.pos.y - 17.0 / 2.0 - CENTER_OFFSET,
            WHITE,
            DrawTextureParams {
                rotation: self.angle, 
                pivot: Some(self.pos),
                dest_size: Some(Vec2::new(17.0, 17.0)),
                ..Default::default()
            },
        );

        // Draw backpack
        draw_texture_ex(
            assets.get_texture("backpack.png"),
            self.pos.x - 17.0 / 2.0 + CENTER_OFFSET,
            self.pos.y - 17.0 / 2.0 - CENTER_OFFSET,
            WHITE,
            DrawTextureParams {
                rotation: self.angle,
                pivot: Some(self.pos),
                dest_size: Some(Vec2::new(17.0, 17.0)),
                ..Default::default()
            },
        );
    }
}

impl PlayerClothes {
}
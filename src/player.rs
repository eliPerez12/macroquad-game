use macroquad::prelude::*;

pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub health: f32,
    pub stamina: f32,
    pub stamina_state: PlayerStaminaState,
    pub movement_state: PlayerMovementState,
    pub angle: f32,
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

// Impls!

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
        }
    }

    // Function to handle player movements
    pub fn handle_player_movements(&mut self) {
        self.handle_movement_state();
        self.handle_velocity();
        self.handle_stamina();
        self.apply_velocity();

        self.update_angle_to_mouse();
    }

    fn handle_velocity(&mut self) {

        // Determine max velocity based on running state
        let player_max_vel: f32 = {
            if self.movement_state == PlayerMovementState::Sprinting && self.stamina_state == PlayerStaminaState::Normal {
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
        if self.movement_state == PlayerMovementState::Sprinting && self.stamina <= 0.0 { // Enter recovering stamina state
            self.stamina_state = PlayerStaminaState::Recovering
        }
        if self.movement_state == PlayerMovementState::Sprinting  && self.stamina_state == PlayerStaminaState::Normal {
            self.stamina = (self.stamina - 0.2).max(0.0);
        }
        else if self.stamina < Player::MAX_STAMINA {
            self.stamina = (self.stamina + 0.1).min(Player::MAX_STAMINA);
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
        self.pos += self.vel;
    }

    // Function to update the player's angle towards the mouse position
    pub fn update_angle_to_mouse(&mut self) {
        let mouse_pos: Vec2 = mouse_position().into();
        let screen_center = Vec2::new(screen_width() / 2.0, screen_height() / 2.0);
        let mouse_dist_center = mouse_pos - screen_center;
        self.angle = f32::atan2(-mouse_dist_center.x, mouse_dist_center.y);
    }

    // Checks if any keys are down that would move the player
    fn is_moving(&self) -> bool {
        if is_key_down(KeyCode::W) {
            return true
        }
        if is_key_down(KeyCode::S) {
            return true
        }
        if is_key_down(KeyCode::A) {
            return true
        }
        if is_key_down(KeyCode::D) {
            return true
        } 
        false
    }
}



// Drawing logic
impl Player {

    pub fn draw(&self, player_sprite: &Texture2D) {
        // Draw player shadow
        draw_circle(
            self.pos.x + 0.50,
            self.pos.y + 0.50,
            3.2,
            Color::from_rgba(0, 0, 0, 70),
        );

        // Draw player
        draw_texture_ex(
            &player_sprite,
            self.pos.x - 5.5,
            self.pos.y - 5.5,
            WHITE,
            DrawTextureParams {
                rotation: self.angle,
                pivot: Some(self.pos),
                dest_size: Some(Vec2::new(11.0, 11.0)),
                ..Default::default()
            },
        );
    }
}
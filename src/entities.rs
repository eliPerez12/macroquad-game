use macroquad::prelude::*;

use crate::{assets::Assets, camera::GameCamera, player::Player};

pub struct Dummy {
    pub pos: Vec2,
    pub angle: f32,
}

impl Dummy {
    pub fn draw(&self, assets: &Assets, player: &Player) {
        let is_in_view = player.is_in_view(self.pos);
        if is_in_view == 0 {
            return;
        };
        const CENTER_OFFSET: f32 = 1.0 / 6.0;
        // Draw player shadow
        draw_circle(
            self.pos.x + CENTER_OFFSET + 0.3,
            self.pos.y - CENTER_OFFSET + 0.3,
            3.2,
            Color::from_rgba(0, 0, 0, 70),
        );

        let player_texture = assets.get_texture("blue_soldier.png");
        let color = match is_in_view {
            1 => Color::from_rgba(200, 100, 200, 255),
            2 => Color::from_rgba(220, 220, 220, 255),
            3 => WHITE,
            _ => unreachable!(),
        };

        // Draw player
        draw_texture_ex(
            &player_texture,
            self.pos.x - 17.0 / 2.0 + CENTER_OFFSET,
            self.pos.y - 17.0 / 2.0 - CENTER_OFFSET,
            color,
            DrawTextureParams {
                rotation: self.angle, // Correction to make the gun face the mouse more accruate
                pivot: Some(self.pos),
                dest_size: Some(Vec2::new(17.0, 17.0)),
                ..Default::default()
            },
        );
    }
}

impl Dummy {
    // Function to update the player's angle towards the mouse position
    pub fn turn_to_face(&mut self, camera: &GameCamera, pos: Vec2) {
        let mouse_pos: Vec2 = camera.world_to_screen(pos);
        let screen_center = camera.world_to_screen(self.pos);
        let mouse_dist_center = mouse_pos - screen_center;
        self.angle = f32::atan2(-mouse_dist_center.x, mouse_dist_center.y);
    }
}

pub struct Bullet {
    pub pos: Vec2,
    pub vel: f32,
    pub angle: f32,
    pub hit_something: bool,
}

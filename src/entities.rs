use crate::{assets::Assets, camera::GameCamera, player::Player, world::TileMap};
use macroquad::{
    audio::{play_sound, PlaySoundParams},
    prelude::*,
};

pub struct Dummy {
    pub pos: Vec2,
    pub angle: f32,
}

pub struct Bullet {
    pub pos: Vec2,
    pub vel: f32,
    pub angle: f32,
    pub hit_something: bool,
}

pub fn handle_shooting(
    bullets: &mut Vec<Bullet>,
    assets: &Assets,
    player: &Player,
    camera: &GameCamera,
    tile_map: &TileMap,
) {
    let is_shooting = (is_mouse_button_pressed(MouseButton::Left) | is_key_pressed(KeyCode::Space))
        && is_mouse_button_down(MouseButton::Right);

    if is_shooting {
        for _ in 0..player.gun.bullets_per_shot {
            let bullet_speed = player.gun.bullet_speed
                + rand::gen_range(-player.gun.bullet_spread, player.gun.bullet_spread); // Apply speed spread

            let mouse_pos: Vec2 = mouse_position().into();
            let mouse_dist_center = mouse_pos - camera.world_to_screen(player.pos);
            let angle = f32::atan2(mouse_dist_center.x, mouse_dist_center.y);
            let angle =
                angle + rand::gen_range(-player.gun.bullet_spread, player.gun.bullet_spread); // Apply angular spread

            bullets.push(Bullet {
                pos: player.pos,
                vel: bullet_speed,
                hit_something: false,
                angle,
            });
        }
        play_sound(
            assets.get_sound("sawed_shotgun.wav"),
            PlaySoundParams::default(),
        );
    }

    for bullet in &mut *bullets {
        let drag = 0.15 * get_frame_time() * 60.0;
        if bullet.vel >= 0.0 {
            bullet.vel -= drag;
            bullet.vel = bullet.vel.max(0.0);
        } else {
            bullet.vel += drag;
            bullet.vel = bullet.vel.min(0.0);
        }
        bullet.pos += Vec2::new(
            f32::sin(bullet.angle) * bullet.vel,
            f32::cos(bullet.angle) * bullet.vel,
        ) * get_frame_time()
            * 60.0;

        if bullet.vel.abs() < 1.0 {
            bullet.hit_something = true
        }
    }
    bullets.retain(|bullet| !bullet.hit_something);
    bullets.retain(|bullet| !tile_map.point_collides_with_tile(bullet.pos));
}

impl Dummy {
    pub fn draw(&self, assets: &Assets, _player: &Player) {
        let is_in_view = 3; // TODO: REMOVE
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

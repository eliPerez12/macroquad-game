use crate::{assets::Assets, camera::GameCamera, player::Player, tile_map::TileMap};
use macroquad::{
    audio::{play_sound, PlaySoundParams},
    prelude::*,
};

#[derive(Clone)]
pub struct Bullet {
    pub pos: Vec2,
    pub vel: f32,
    pub angle: f32,
    pub hit_something: bool,
    pub last_pos: Vec2,
}


pub struct EntityManager {
    pub other_players: Vec<Option<Player>>,
    pub other_player_index: u32,
    pub bullets: Vec<Bullet>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            other_players: vec![],
            other_player_index: 0,
            bullets: vec![],
        }
    }
    pub fn update(&mut self, player: &Player, camera: &GameCamera) {
        for other_player in self.other_players.iter_mut().flatten() {
            other_player.turn_to_face(player.pos, camera);
        }
    }

    pub fn point_collides_with_entity(&self, point: Vec2) -> bool {
        for player in self.other_players.iter().flatten() {
            if player.get_hitbox().contains(point) {
                return true;
            }
        }
        false
    }

    pub fn draw_entities(
        &self,
        assets: &Assets,
        player: &Player,
        tile_map: &TileMap,
    ) {
        // Get tiles visible to camera
        let visible_tiles = tile_map.find_tiles(
            player.get_player_rays(std::f32::consts::PI, crate::world::LINE_LENGTH),
            crate::world::LINE_LENGTH / 8.0 * 1.0,
            player.pos,
        );
        // Draw bullets
        for bullet in &self.bullets {
            //draw_circle(bullet.pos.x, bullet.pos.y, 0.2, WHITE);
            draw_line(bullet.pos.x, bullet.pos.y, bullet.last_pos.x, bullet.last_pos.y, 0.18, WHITE);
        }
        // Draw players
        for other_player in self.other_players.iter().flatten() {
            let dist_to_player = {
                let dx = other_player.pos.x - player.pos.x;
                let dy = other_player.pos.y - player.pos.y;
                (dx * dx + dy * dy).sqrt()
            };
            if visible_tiles.contains(&(
                (other_player.pos.x / 8.0) as u16,
                (other_player.pos.y / 8.0) as u16,
            )) || dist_to_player < 18.0
            {
                other_player.draw(assets);
            }
        }
    }
    pub fn draw_entity_hitboxes(&self) {
        for other_player in self.other_players.iter().flatten() {
            other_player.draw_hitbox();
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.other_players
            .insert(self.other_player_index as usize, Some(player));
    }

    pub fn handle_shooting(
        &mut self,
        assets: &Assets,
        player: &Player,
        camera: &GameCamera,
        tile_map: &TileMap,
    ) {
        let is_shooting = (is_mouse_button_pressed(MouseButton::Left)
            | is_key_pressed(KeyCode::Space))
            && is_mouse_button_down(MouseButton::Right);

        if is_shooting {
            for _ in 0..player.inventory.gun.bullets_per_shot {
                let bullet_speed = player.inventory.gun.bullet_speed
                    + rand::gen_range(-player.inventory.gun.bullet_spread, player.inventory.gun.bullet_spread); // Apply speed spread

                let mouse_pos: Vec2 = mouse_position().into();
                let mouse_dist_center = mouse_pos - camera.world_to_screen(player.pos);
                let angle = f32::atan2(mouse_dist_center.x, mouse_dist_center.y);

                let (barrel_offset_x, barrel_offset_y) =
                    (player.inventory.gun.barrel_offset.x, player.inventory.gun.barrel_offset.y);

                let bullet_pos = Vec2 {
                    x: player.pos.x + barrel_offset_x * -angle.cos()
                        - barrel_offset_y * angle.sin(),
                    y: player.pos.y
                        + barrel_offset_x * angle.sin()
                        + barrel_offset_y * -angle.cos(),
                };
                let mut new_angle = angle;
                let dist_from_player = (mouse_dist_center.x * mouse_dist_center.x
                    + mouse_dist_center.y * mouse_dist_center.y)
                    .sqrt();
                if dist_from_player > 80.0 {
                    let mouse_dist_center = mouse_pos - camera.world_to_screen(bullet_pos);
                    new_angle = f32::atan2(mouse_dist_center.x, mouse_dist_center.y);
                }

                new_angle += rand::gen_range(-player.inventory.gun.bullet_spread, player.inventory.gun.bullet_spread);

                self.bullets.push(Bullet {
                    pos: bullet_pos,
                    vel: bullet_speed,
                    hit_something: false,
                    angle: new_angle,
                    last_pos: bullet_pos
                });
            }
            play_sound(
                assets.get_sound("sawed_shotgun.wav"),
                PlaySoundParams::default(),
            );
        }

        for bullet in &mut *self.bullets {
            bullet.last_pos = bullet.pos;
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

            if bullet.vel.abs() < 0.60 {
                bullet.hit_something = true
            }
        }
        let mut new_bullets = self.bullets.clone();
        new_bullets.retain(|bullet| !bullet.hit_something && !self.point_collides_with_entity(bullet.pos));
        self.bullets = new_bullets;
        self.bullets
            .retain(|bullet| !tile_map.point_collides_with_tile(bullet.pos));
    }
}
use crate::{
    assets::Assets,
    camera::GameCamera,
    player::Player,
    tile_map::{LineSegment, TileMap},
};
use macroquad::prelude::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Bullet {
    pub pos: Vec2,
    pub vel: f32,
    pub angle: f32,
    pub hit_something: Option<Vec2>,
    pub last_pos: Vec2,
}

pub struct Grenade {
    pub pos: Vec2,
    pub rotation: f32,
    pub rotation_speed: f32,
    pub fuse_time: f32,
}

impl Grenade {
    pub const MAX_FUSE_TIME: f32 = 5.0;
    pub const MAX_ROTATION_SPEED: f32 = 0.15;
}

pub struct EntityManager {
    pub other_players: Vec<Option<Player>>,
    pub other_player_index: u32,
    pub bullets: Vec<Bullet>,
    pub grenades: Vec<Grenade>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            other_players: vec![],
            other_player_index: 0,
            bullets: vec![],
            grenades: vec![],
        }
    }

    pub fn update(&mut self, player: &Player, camera: &GameCamera) {
        for other_player in self.other_players.iter_mut().flatten() {
            other_player.turn_to_face(player.pos, camera);
        }
    }

    fn line_collides_with_entity(&self, line: &LineSegment) -> Option<Vec2> {
        for player in self.other_players.iter().flatten() {
            if let Some(intersect) = line.line_intersects_rect(player.get_hitbox()) {
                return Some(intersect);
            }
        }
        None
    }

    pub fn draw_entities(&mut self, assets: &Assets, player: &Player, tile_map: &TileMap) {
        let is_visible = |pos: Vec2, visible_tiles: &HashSet<(u16, u16)>| -> bool {
            let dist_to_player = {
                let dx = pos.x - player.pos.x;
                let dy = pos.y - player.pos.y;
                (dx * dx + dy * dy).sqrt()
            };
            visible_tiles.contains(&((pos.x / 8.0) as u16, (pos.y / 8.0) as u16))
                || dist_to_player < 18.0
        };

        // Get tiles visible to camera
        let visible_tiles = tile_map.find_tiles(
            player.get_player_rays(std::f32::consts::PI, crate::world::LINE_LENGTH),
            crate::world::LINE_LENGTH / 8.0 * 1.0,
            player.pos,
        );
        // Draw bullets
        for bullet in &self.bullets {
            if is_visible(bullet.pos, &visible_tiles) {
                match bullet.hit_something {
                    None => {
                        draw_line(
                            bullet.pos.x,
                            bullet.pos.y,
                            bullet.last_pos.x,
                            bullet.last_pos.y,
                            0.18,
                            WHITE,
                        );
                    },
                    Some(intersect) => {
                        dbg!(intersect);
                        draw_circle(intersect.x, intersect.y, 1.0, RED);
                    },
                    
                        //draw_line(
                    //     intersect.x,
                    //     intersect.y,
                    //     bullet.last_pos.x,
                    //     bullet.last_pos.y,
                    //     0.18,
                    //     WHITE,
                    // )
                }
            };
        }
        // Draw grenades
        for grenade in &self.grenades {
            if is_visible(grenade.pos, &visible_tiles) {
                draw_texture_ex(
                    &assets.get_texture("grenade_unpinned.png"),
                    grenade.pos.x,
                    grenade.pos.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(14.0, 14.0)),
                        rotation: grenade.rotation,
                        ..Default::default()
                    },
                )
            }
        }
        // Draw players
        for other_player in self.other_players.iter().flatten() {
            if is_visible(other_player.pos, &visible_tiles) {
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

    pub async fn handle_shooting(
        &mut self,
        assets: &Assets,
        player: &Player,
        camera: &GameCamera,
        tile_map: &TileMap,
    ) {
        let is_shooting = (is_mouse_button_pressed(MouseButton::Left)
            | is_key_pressed(KeyCode::Space))
            && is_mouse_button_down(MouseButton::Right);

        // Remove old bullets that hit somthing
        self.bullets.retain(|bullet| bullet.hit_something.is_none());

        // Handle spawning bullets
        if is_shooting {
            for _ in 0..player.inventory.gun.bullets_per_shot {
                let bullet_speed = player.inventory.gun.bullet_speed
                    + rand::gen_range(
                        -player.inventory.gun.bullet_spread,
                        player.inventory.gun.bullet_spread,
                    ); // Apply speed spread

                let mouse_pos: Vec2 = mouse_position().into();
                let mouse_dist_center = mouse_pos - camera.world_to_screen(player.pos);
                let angle = f32::atan2(mouse_dist_center.x, mouse_dist_center.y);

                let (barrel_offset_x, barrel_offset_y) = (
                    player.inventory.gun.barrel_offset.x,
                    player.inventory.gun.barrel_offset.y,
                );

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

                new_angle += rand::gen_range(
                    -player.inventory.gun.bullet_spread,
                    player.inventory.gun.bullet_spread,
                );

                self.bullets.push(Bullet {
                    pos: bullet_pos,
                    vel: bullet_speed,
                    hit_something: None,
                    angle: new_angle,
                    last_pos: bullet_pos,
                });
            }
            let sound_name = format!("{}{}", player.inventory.gun.name, "_shooting.wav");
            assets.play_sound(&sound_name);
        }

        // Grenades
        for grenade in &mut self.grenades {
            let fuse_percent = grenade.fuse_time / Grenade::MAX_FUSE_TIME;
            grenade.rotation_speed = Grenade::MAX_ROTATION_SPEED * fuse_percent;
            grenade.rotation += grenade.rotation_speed;
            grenade.fuse_time -= get_frame_time();
        }

        // Bullets
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
        }
        let mut new_bullets = self.bullets.clone();
        new_bullets.iter_mut().for_each(|bullet| {
            let line = &LineSegment {
                x1: bullet.last_pos.x,
                x2: bullet.pos.x,
                y1: bullet.last_pos.y,
                y2: bullet.pos.y,
            };
            if bullet.vel.abs() <= 0.00 {
                bullet.hit_something = Some(bullet.pos);
            }
            else if let Some(intersect) = tile_map.line_collides_with_tile(line) {
                bullet.hit_something = Some(intersect);
            }
            else if let Some(intersect) = self.line_collides_with_entity(line) {
                bullet.hit_something = Some(intersect);
            }
        });
        self.bullets = new_bullets;
        self.grenades.retain(|grenade| grenade.fuse_time > 0.0);
    }
}

use macroquad::prelude::*;
use camera::*;

mod camera;

fn get_src_rect(atlas_grid_x: i32, atlas_grid_y: i32) -> Rect {
    Rect {
        x: atlas_grid_x as f32 * 8.0,
        y: atlas_grid_y as f32 * 8.0,
        w: 8.0,
        h: 8.0,
    }
}

fn draw_rect(rect: &Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
}

struct Player {
    pos: Vec2,
    vel: Vec2,
}

impl Player {
    fn handle_player_movements(&mut self) {
        const PLAYER_ACC: f32 = 0.08;
        const PLAYER_DEACC:  f32 = 0.05;
        let player_max_vel: f32 = match is_key_down(KeyCode::LeftShift) {
            true => 0.55,
            false => 0.33
        };

        if self.vel.x > 0.0 {
            self.vel.x -= PLAYER_DEACC;
            self.vel.x = self.vel.x.max(0.0);
        }
        if self.vel.y > 0.0 {
            self.vel.y -= PLAYER_DEACC;
            self.vel.y = self.vel.y.max(0.0);
        }
        if self.vel.x < 0.0 {
            self.vel.x += PLAYER_DEACC;
            self.vel.x = self.vel.x.min(0.0);
        }
        if self.vel.y < 0.0 {
            self.vel.y += PLAYER_DEACC;
            self.vel.y = self.vel.y.min(0.0);
        }

        if is_key_down(KeyCode::W) {
            self.vel.y -= PLAYER_ACC;
        }
        if is_key_down(KeyCode::S) {
            self.vel.y += PLAYER_ACC;
        }
        if is_key_down(KeyCode::A) {
            self.vel.x -= PLAYER_ACC;
        }
        if is_key_down(KeyCode::D) {
            self.vel.x += PLAYER_ACC;
        }

        if self.vel.x > player_max_vel {
            self.vel.x = player_max_vel;
        }
        if self.vel.x < -player_max_vel {
            self.vel.x = -player_max_vel;
        }
        if self.vel.y > player_max_vel {
            self.vel.y = player_max_vel;
        }
        if self.vel.y < -player_max_vel {
            self.vel.y = -player_max_vel;
        }

        self.pos += self.vel;
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut camera = GameCamera {
        ..Default::default()
    };

    let tiles_sheet =
        Texture2D::from_image(&load_image("assets/jawbreaker_tiles.png").await.unwrap());
    tiles_sheet.set_filter(FilterMode::Nearest);

    let mut player = Player {
        pos: Vec2::ZERO,
        vel: Vec2::ZERO,
    };

    // Main game loop
    loop {
        // Update Game
        player.handle_player_movements();

        if is_key_down(KeyCode::Q) {
            camera.zoom *= 1.01;
        }
        if is_key_down(KeyCode::E) {
            camera.zoom /= 1.01;
        }

        camera.pan_to_target(player.pos + Vec2::new(4.0, 4.0));

        // Draw in world space
        set_camera(&mut camera);
        clear_background(BLACK);

        for y in 0..20 {
            for x in 0..20 {
                draw_texture_ex(
                    &tiles_sheet,
                    8.0 * x as f32,
                    8.0 * y as f32,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(8.0, 8.0)),
                        source: Some(get_src_rect(1, 0)),
                        ..Default::default()
                    },
                );
            }
        }
        // Draw player
        draw_rect(&Rect::new(player.pos.x, player.pos.y, 8.0, 8.0), RED);

        // Draw in screen space
        set_default_camera();
        draw_text(&get_fps().to_string(), 50.0, 50.0, 100.0, WHITE);

        next_frame().await;
    }
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        ..Default::default()
    }
}

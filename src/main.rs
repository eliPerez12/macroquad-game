use macroquad::prelude::*;

fn get_src_rect(grid_x: i32, grid_y: i32) -> Rect {
    Rect {
        x: grid_x as f32 * 8.0,
        y: grid_y as f32 * 8.0,
        w: 8.0,
        h: 8.0,
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

// draw texture in grid

fn draw_rect(rect: &Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color)
}

fn default_camera_zoom() -> Vec2 {
    vec2(1.0 / screen_width(), 1.0 / screen_height())
}

struct Player {
    pos: Vec2,
    vel: Vec2,
}

#[macroquad::main(conf)]
async fn main() {
    let mut camera = Camera2D {
        zoom: default_camera_zoom(),
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
        clear_background(BLACK);

        let player_acc = 0.305;
        let player_max_vel = 0.8;

        if player.vel.x > 0.0 {
            player.vel.x -= 0.3;
            player.vel.x = player.vel.x.max(0.0);
        }
        if player.vel.y > 0.0 {
            player.vel.y -= 0.3;
            player.vel.y = player.vel.y.max(0.0);
        }
        if player.vel.x < 0.0 {
            player.vel.x += 0.3;
            player.vel.x = player.vel.x.min(0.0);
        }
        if player.vel.y < 0.0 {
            player.vel.y += 0.3;
            player.vel.y = player.vel.y.min(0.0);
        }

        if is_key_down(KeyCode::W) {
            player.vel.y -= player_acc;
        }
        if is_key_down(KeyCode::S) {
            player.vel.y += player_acc;
        }
        if is_key_down(KeyCode::A) {
            player.vel.x -= player_acc;
        }
        if is_key_down(KeyCode::D) {
            player.vel.x += player_acc;
        }
        if is_key_down(KeyCode::Q) {
            camera.zoom *= 1.01;
        }
        if is_key_down(KeyCode::E) {
            camera.zoom /= 1.01;
        }

        if player.vel.x > player_max_vel {
            player.vel.x = player_max_vel;
        }
        if player.vel.x < -player_max_vel {
            player.vel.x = -player_max_vel;
        }
        if player.vel.y > player_max_vel {
            player.vel.y = player_max_vel;
        }
        if player.vel.y < -player_max_vel {
            player.vel.y = -player_max_vel;
        }

        player.pos += player.vel;
        let camera_dist_from_player = camera.target - player.pos;
        camera.target -= camera_dist_from_player / 17.5;

        // Draw in world space
        set_camera(&mut camera);
        for y in 0..20 {
            for x in 0..20 {
                draw_texture_ex(
                    &tiles_sheet,
                    8.0 * x as f32,
                    8.0 * y as f32,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(8.0, 8.0)),
                        source: Some(get_src_rect(0, 0)),
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

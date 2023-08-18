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

fn draw_rect_lines(rect: &Rect, thickness: f32, color: Color) {
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, thickness, color)
}

async fn load_texture(path: &str) -> Result<Texture2D, macroquad::Error> {
    let texture = Texture2D::from_image(&load_image(path).await?);
    texture.set_filter(FilterMode::Nearest);
    Ok(texture)
}

struct Player {
    pos: Vec2,
    vel: Vec2,
    stamina: f32,
    angle: f32, // in radians
}

impl Player {
    const MAX_STAMINA: f32 = 100.0;
    
    fn handle_player_movements(&mut self) {
        const PLAYER_ACC: f32 = 0.1;
        const PLAYER_DEACC: f32 = 0.05;

        let player_max_vel: f32 = if is_key_down(KeyCode::LeftShift) {
            0.45
        } else {
            0.28
        };

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

        // Normalize the velocity to make sure the player moves at a constant speed
        let magnitude = (self.vel.x.powi(2) + self.vel.y.powi(2)).sqrt();
        if magnitude > player_max_vel {
            self.vel.x = (self.vel.x / magnitude) * player_max_vel;
            self.vel.y = (self.vel.y / magnitude) * player_max_vel;
        }

        // Deacceleration logic
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

        self.pos += self.vel;
    }

    fn update_angle_to_mouse(&mut self) {
        let mouse_pos: Vec2 = mouse_position().into();
        let screen_center = Vec2::new(screen_width()/2.0, screen_height()/2.0);
        let mouse_dist_center = mouse_pos - screen_center;
        let angle = f32::atan2(-mouse_dist_center.x, mouse_dist_center.y);
        self.angle = angle;
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut camera = GameCamera {
        ..Default::default()
    };

    let _tiles_sheet = load_texture("assets/tiles.png").await.unwrap();
    let player_sprite = load_texture("assets/soldier.png").await.unwrap();
    let example_world = load_texture("assets/sample.png").await.unwrap();

    let mut player = Player {
        pos: Vec2::ZERO,
        vel: Vec2::ZERO,
        stamina: Player::MAX_STAMINA,
        angle: 0.0,
    };

    // Main game loop
    loop {
        // Update Game
        player.handle_player_movements();
        player.update_angle_to_mouse();

        if is_key_down(KeyCode::Q) {
            camera.target_zoom *= 1.01;
        }
        if is_key_down(KeyCode::E) {
            camera.target_zoom /= 1.01;
        }


        camera.set_camera_zoom();
        camera.pan_to_target(player.pos + Vec2::new(0.0, 0.0));

        // Draw in world space
        set_camera(&mut camera);
        clear_background(BLACK);

        draw_texture(&example_world, 0.0, 0.0, WHITE);

        // Draw player
        draw_circle(player.pos.x + 0.50, player.pos.y + 0.50, 3.2, Color::from_rgba(0, 0, 0, 70));

        draw_texture_ex(&player_sprite, player.pos.x - 5.5, player.pos.y - 5.5, WHITE, DrawTextureParams {
            rotation: player.angle,
            pivot: Some(player.pos + Vec2::new(0.0, 0.0)),
            dest_size: Some(Vec2::new(11.0, 11.0)),
        ..Default::default()}
        );

        // Draw in screen space
        set_default_camera();
        draw_text(&get_fps().to_string(), 50.0, 50.0, 100.0, WHITE);


        let health_bar = Rect {
            x: screen_width()/20.0,
            y: screen_height() - screen_height()/12.0,
            w: screen_width()/5.0,
            h: screen_height()/80.0
        };

        // 
        let stamina_bar = Rect {
            x: screen_width()/20.0,
            y: screen_height() - screen_height()/15.0,
            w: screen_width()/5.0,
            h: screen_height()/80.0
        };

        draw_rect(&health_bar, Color::from_rgba(255, 30, 30, 180));
        draw_rect_lines(&health_bar, 3.0, BLACK);

        draw_rect(&stamina_bar, Color::from_rgba(50, 50, 50, 180));
        draw_rect_lines(&stamina_bar, 3.0, BLACK);

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

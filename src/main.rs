use macroquad::prelude::*;
use camera::*;
use std::collections::HashMap;
use player::*;

mod camera;
mod player;

fn draw_rect(rect: &Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
}

fn draw_rect_lines(rect: &Rect, thickness: f32, color: Color) {
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, thickness, color)
}

struct Bullet {
    pos: Vec2,
    vel: f32,
    angle: f32,
    hit_something: bool
}

pub struct Assets {
    textures: HashMap<String, Texture2D>,
}

impl Assets {
    fn get_texture(&self, texture: &str) -> &Texture2D{
        self.textures.get(texture).unwrap()
    }

    async fn load_texture(path: &str) -> Result<Texture2D, macroquad::Error> {
        let texture = Texture2D::from_image(&load_image(path).await?);
        texture.set_filter(FilterMode::Nearest);
        Ok(texture)
    }

    // Stack based search through assets folder, and loads in all assets
    async fn load_all_assets() -> Self {
        let mut textures: HashMap<String, Texture2D> = HashMap::new();
        let mut dirs_to_explore = vec![std::path::PathBuf::from("assets")];
    
        while let Some(dir) = dirs_to_explore.pop() {
            for entry in std::fs::read_dir(dir).unwrap(){
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() && path.to_str().unwrap() != "temp"{
                    dirs_to_explore.push(path);
                } else if path.is_file() {
                    let path_str = path.to_string_lossy().to_string();
                    if path_str.ends_with(".png") {
                        let key_path_str = path_str.split("/").last().unwrap();
                        textures.insert(key_path_str.to_string(), Assets::load_texture(&path_str).await.unwrap());
                    }
                }
            }
        }
        Assets { textures }
    }
}


#[macroquad::main(conf)]
async fn main() {
    let mut camera = GameCamera {
        ..Default::default()
    };
    
    let assets = Assets::load_all_assets().await;
    let example_world = Assets::load_texture("assets/temp/sample.png").await.unwrap();

    let mut player = Player::new();
    let mut bullets: Vec<Bullet> = vec![];

    // Main game loop
    loop {
        // Update Game
        player.handle_player_movements(&camera);

        camera.handle_controls();
        camera.pan_to_target(player.pos);

        if (is_mouse_button_pressed(MouseButton::Left) | is_key_pressed(KeyCode::Space)) && is_mouse_button_down(MouseButton::Right) {
            for _ in 0..8 {
                let bullet_spread = 0.15;
                let bullet_speed = 5.3 + rand::gen_range(-bullet_spread, bullet_spread); // Apply speed spread

                let mouse_pos: Vec2 = mouse_position().into();
                let mouse_dist_center = mouse_pos - camera.world_to_screen(player.pos);
                let angle = f32::atan2(mouse_dist_center.x, mouse_dist_center.y);
                let angle = angle + rand::gen_range(-bullet_spread, bullet_spread); // Apply angular spread
            
                bullets.push(Bullet {
                    pos: player.pos,
                    vel: bullet_speed,
                    hit_something: false,
                    angle
                });
            }
        }
        
        for bullet in &mut bullets {
            let drag = 0.1;
            if bullet.vel >= 0.0 {
                bullet.vel -= drag;
                bullet.vel = bullet.vel.max(0.0);
            } else {
                bullet.vel += drag;
                bullet.vel = bullet.vel.min(0.0);
            }
            bullet.pos += Vec2::new(
                f32::sin(bullet.angle) * bullet.vel,
                f32::cos(bullet.angle) * bullet.vel
            ) * get_frame_time() * 60.0;
        
            if bullet.vel.abs() < 1.0 {
                bullet.hit_something = true
            }
        }
        
        bullets.retain(|bullet| !bullet.hit_something);
        
        // Draw in world space
        set_camera(&mut camera);
        clear_background(BLACK);

        // Draws example world
        draw_texture(&example_world, 0.0, 0.0, WHITE);
        player.draw(&assets);

        // Bullets
        for bullet in &bullets {
            draw_circle(bullet.pos.x, bullet.pos.y, 0.2, WHITE);
        }

        // Draw in screen space
        set_default_camera();
        draw_text(&get_fps().to_string(), 50.0, 50.0, 100.0, WHITE);

        let health_bar = Rect {
            x: screen_width() / 20.0,
            y: screen_height() - screen_height() / 11.0,
            w: screen_width() / 5.0,
            h: screen_height() / 50.0,
        };

        let stamina_bar = Rect {
            x: screen_width() / 20.0,
            y: screen_height() - screen_height() / 15.0,
            w: screen_width() / 5.0,
            h: screen_height() / 80.0,
        };

        let filled_health_bar = {
            let mut filled_bar = health_bar.clone();
            filled_bar.w = player.health * filled_bar.w / 100.0;
            filled_bar
        };

        let filled_stamina_bar = {
            let mut filled_bar = stamina_bar.clone();
            filled_bar.w = player.stamina * filled_bar.w / 100.0;
            filled_bar
        };

        let stamina_bar_color = match player.stamina_state {
            PlayerStaminaState::Normal => Color::from_rgba(60, 60, 60, 180),
            PlayerStaminaState::Recovering => Color::from_rgba(100, 100, 100, 200)
        };

        draw_rect(&filled_health_bar, Color::from_rgba(255, 30, 30, 200));
        draw_rect_lines(&health_bar, 3.0, BLACK);

        draw_rect(&filled_stamina_bar, stamina_bar_color);
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

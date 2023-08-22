use macroquad::{prelude::*, audio::*};
use camera::*;
use assets::Assets;
use player::Player;
use ui::render_ui;

mod camera;
mod player;
mod assets;
mod ui;

struct Dummy {
    pos: Vec2,
    angle: f32,
}

impl Dummy {
    pub fn draw(&self, assets: &Assets, player: &Player) {
        let is_in_view = player.is_in_view(self.pos);
        dbg!(player.is_in_view(self.pos));
        if is_in_view == 0 { return };
        const CENTER_OFFSET: f32 = 1.0/6.0;
        // Draw player shadow
        draw_circle(
            self.pos.x + CENTER_OFFSET + 0.3,
            self.pos.y - CENTER_OFFSET + 0.3,
            3.2,
            Color::from_rgba(0, 0, 0, 70),
        );

        let player_texture = assets.get_texture("blue_soldier2.png");
        let color = match is_in_view {
            1 => Color::from_rgba(200, 100,200, 255),
            2 => Color::from_rgba(220, 220, 220, 255),
            3 => WHITE,
            _ => unreachable!()
        };

        // Draw player
        draw_texture_ex(
            &player_texture,
            self.pos.x - 17.0/2.0 + CENTER_OFFSET,
            self.pos.y - 17.0/2.0 - CENTER_OFFSET,
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

struct Bullet {
    pos: Vec2,
    vel: f32,
    angle: f32,
    hit_something: bool,
}


#[macroquad::main(conf)]
async fn main() {
    let mut camera = GameCamera::new();
    
    let assets = Assets::new().await;
    let example_world = assets.get_texture("sample.png");

    let mut player = Player::new();
    player.pos = Vec2::new(60.0, 50.0);

    let dummy1 = Dummy {pos: vec2(100.0, 100.0), angle: 0.0};
    let mut bullets: Vec<Bullet> = vec![];
    let mut camera_state = true;

    // Main game loop
    loop {

        if is_key_pressed(KeyCode::P) {
            camera_state = !camera_state;
        }

        // Update Game
        player.handle_player_movements(&camera);
        camera.handle_controls();
        camera.pan_to_target(if camera_state {player.pos} else {Vec2::ZERO});


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
                    angle,
                });
            }
            play_sound(assets.get_sound("shotgun00.wav"), PlaySoundParams { looped: false, volume: 0.3 });
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

        // Draw player and dummys
        player.draw(&assets);
        dummy1.draw(&assets, &player);

        // Draw shadows
        for y in 0..40 {
            for x in 0..40 {
                let tile_pos = Vec2::new(x as f32 * 8.0 + 4.0, y as f32 * 8.0 + 4.0);

                // Draw shadow if tile is outside the player's field of view
                if player.is_in_view(tile_pos) == 0{
                    draw_rectangle(x as f32 * 8.0, y as f32 * 8.0, 8.0, 8.0, Color::from_rgba(0, 0, 0, 50));
                }
                else if player.is_in_view(tile_pos) == 1 {
                    draw_rectangle(x as f32 * 8.0, y as f32 * 8.0, 8.0, 8.0, Color::from_rgba(0, 0, 0, 35));
                }
                else if player.is_in_view(tile_pos) == 2 {
                    draw_rectangle(x as f32 * 8.0, y as f32 * 8.0, 8.0, 8.0, Color::from_rgba(0, 0, 0, 25));
                }
            }
        }


        // Bullets
        for bullet in &bullets {
            draw_circle(bullet.pos.x, bullet.pos.y, 0.2, WHITE);
        }

        // Draw in screen space
        set_default_camera();
        draw_text(&get_fps().to_string(), 50.0, 50.0, 100.0, WHITE);

        // Rendering UI
        render_ui(&player);

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

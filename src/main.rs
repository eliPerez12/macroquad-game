use camera::GameCamera;
use macroquad::{prelude::*, audio::*, miniquad::conf::Icon};
use assets::Assets;
use player::Player;
use entities::*;
use ui::{render_ui, render_debug_ui};
use world::draw_world;

mod maps;
mod world;
mod camera;
mod player;
mod assets;
mod entities;
mod utils;
mod ui;

fn handle_shooting(bullets: &mut Vec<Bullet>, assets: &Assets, player: &Player, camera: &GameCamera) {
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

    for bullet in  &mut *bullets {
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
}

#[macroquad::main(conf)]
async fn main() {
    let mut camera = GameCamera::new();
    let assets = Assets::new().await;
    let world = maps::example_world();
    let mut dummy = Dummy { pos: vec2(2.2 * 8.0, 9.3 * 8.0), angle: 0.0 };
    let mut debug_on = false;

    let mut player = Player::new();
    player.pos = Vec2::new(60.0, 50.0);

    let mut bullets: Vec<Bullet> = vec![];
    // Main game loop
    loop {
        // Update Game
        player.handle_player_movements(&camera);
        handle_shooting(&mut bullets, &assets, &player, &camera);
        dummy.turn_to_face(&camera, player.pos);
        camera.handle_controls();
        camera.pan_to_target(player.pos);

        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::T) {
            debug_on = !debug_on;
        }
        
        // Draw in world space
        set_camera(&mut camera);
        clear_background(BLACK);

        // Draws example world
        draw_world(&world, &assets, &player);

        // Draw dummy
        dummy.draw(&assets, &player);

        // Draw player
        player.draw(&assets);

        // Bullets
        for bullet in &bullets {
            draw_circle(bullet.pos.x, bullet.pos.y, 0.2, WHITE);
        }

        // Draw in screen space
        set_default_camera();

        // Rendering UI
        render_ui(&player);
        if debug_on{render_debug_ui()}
        next_frame().await;
    }
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Macroquad"),
        window_width: 1260,
        window_height: 768,
        fullscreen: false,
        icon: Some(Icon {
            small: [0;1024],
            medium: [0;4096],
            big: [0;16384],
        }),
        ..Default::default()
    }
}

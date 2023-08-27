use macroquad::{prelude::*, audio::*, miniquad::conf::Icon};
use camera::*;
use assets::Assets;
use player::Player;
use entities::*;
use ui::render_ui;

mod camera;
mod player;
mod assets;
mod entities;
mod utils;
mod ui;

fn draw_shadows(player: &Player) {
        // Draw shawdows
        for y in 0..40 {
            for x in 0..40 {
                let tile_pos = Vec2::new(x as f32 * 8.0 + 4.0, y as f32 * 8.0 + 4.0);
                let alpha = match player.is_in_view(tile_pos) {
                    0 => 50,
                    1 => 35,
                    2 => 25,
                    _ => continue,
                };
                draw_rectangle(x as f32 * 8.0, y as f32 * 8.0, 8.0, 8.0, Color::from_rgba(0, 0, 0, alpha));
            }
        }
        

}

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

struct TileMap {
    data: Vec<u8>,
    width: u16,
    height: u16,
}

fn example_world() -> TileMap {
    TileMap {
        width: 10,
        height: 10,
        data: vec![
            0,0,0,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,
            0,0,0,16,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,
            0,0,16,0,0,0,0,0,0,0,
            0,0,0,0,0,0,0,0,0,0,
            0,0,0,16,0,0,0,5,0,0,
            0,16,0,0,0,0,0,0,0,0,
            0,0,0,0,0,16,0,0,0,0,
            0,0,0,0,0,0,0,0,16,0,
        ]
    }
}

fn draw_world(tiles: &TileMap, assets: &Assets) {
    for (tiles_index, tile) in tiles.data.iter().enumerate() {
        let grid_x = (tiles_index as u16 % tiles.width) as f32;
        let grid_y = (tiles_index as u16 / tiles.width) as f32;
        draw_texture_ex(
            assets.get_texture("tiles.png"),
            grid_x * 8.0,
            grid_y * 8.0,
            WHITE, // Make into shadow render later
            DrawTextureParams {
                source: Some(Rect::new((tile % 8) as f32 * 8.0, (tile / 8) as f32 * 8.0, 8.0, 8.0)),
                ..Default::default()
            }
        )
    }
}

#[macroquad::main(conf)]
async fn main() {
    let mut camera = GameCamera::new();
    
    let assets = Assets::new().await;
    let example_world = example_world();

    let mut dummy = Dummy { pos: vec2(2.2 * 8.0, 9.3 * 8.0), angle: 0.0 };

    let mut player = Player::new();
    player.pos = Vec2::new(60.0, 50.0);

    let mut bullets: Vec<Bullet> = vec![];
    let mut camera_state = true;

    // Main game loop
    loop {

        if is_key_pressed(KeyCode::P) {
            camera_state = !camera_state;
        }

        // Update Game
        player.handle_player_movements(&camera);
        handle_shooting(&mut bullets, &assets, &player, &camera);
        dummy.turn_to_face(&camera, player.pos);
        camera.handle_controls();
        camera.pan_to_target(if camera_state {player.pos} else {Vec2::ZERO});
        
        // Draw in world space
        set_camera(&mut camera);
        clear_background(BLACK);

        // Draws example world
        draw_world(&example_world, &assets);
        
        // Draw dummy
        dummy.draw(&assets, &player);
        draw_shadows(&player);
        // Draw player
        player.draw(&assets);

        // Bullets
        for bullet in &bullets {
            draw_circle(bullet.pos.x, bullet.pos.y, 0.2, WHITE);
        }

        // Draw in screen space
        set_default_camera();
        draw_text(
            &get_fps().to_string(),
            50.0,
            50.0, 
            100.0,
            WHITE,
        );

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
        icon: Some(Icon {
            small: [0;1024],
            medium: [0;4096],
            big: [0;16384],
        }),
        ..Default::default()
    }
}

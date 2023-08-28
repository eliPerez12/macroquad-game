use macroquad::{prelude::*, audio::*, miniquad::conf::Icon};
use std::collections::HashSet;
use camera::*;
use assets::Assets;
use player::Player;
use entities::*;
use ui::render_ui;
use utils::is_windows;

mod maps;
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

pub struct TileMap {
    data: Vec<u8>,
    width: u16,
    height: u16,
}

fn draw_tiles(tiles: &TileMap, assets: &Assets, visible_tiles: HashSet<(u16, u16)>) {
    for (tiles_index, tile) in tiles.data.iter().enumerate() {
        let fit_offset = 0.0675;
        let grid_x = tiles_index as u16 % tiles.width;
        let grid_y = tiles_index as u16 / tiles.width;
        let color = match visible_tiles.contains(&(grid_x, grid_y)) {
            true => Color::from_rgba(255, 255, 255, 255),
            false => Color::from_rgba(200, 200, 200, 255),
        };
        draw_texture_ex(
            assets.get_texture("tiles.png"),
            grid_x as f32 * 8.0,
            grid_y as f32 * 8.0,
            color, // Make into shadow render later
            DrawTextureParams {
                source: Some(Rect::new(
                    ((tile - 1) % 8) as f32 * 8.0 + fit_offset/2.0,
                    ((tile - 1) / 8) as f32 * 8.0 + fit_offset/2.0,
                    8.0 - fit_offset,
                    8.0 - fit_offset)
                ),
                dest_size: Some(Vec2::new(8.0, 8.0)),
                ..Default::default()
            }
        );
    }
}

fn find_tiles(origin: Vec2, angle: f32, length: f32, definition: f32) -> HashSet<(u16, u16)> {
    let mut tiles = HashSet::new();
    let adjusted_angle = angle + std::f32::consts::FRAC_PI_2;
    let (mut x, mut y) = (origin.x / 8.0, origin.y / 8.0);
    let (dx, dy) = (adjusted_angle.cos() / definition, adjusted_angle.sin() / definition);
    let loop_count = (length * definition) as i32;

    for _ in 0..loop_count {
        let tile_x = x.floor() as u16;
        let tile_y = y.floor() as u16;
        tiles.insert((tile_x, tile_y));
        x += dx;
        y += dy;
    }
    tiles
}

fn combine_hashsets(hashsets: Vec<HashSet<(u16, u16)>>) -> HashSet<(u16, u16)> {
    let mut combined = HashSet::new();
    for set in hashsets {
        combined.extend(set);
    }
    combined
}

fn draw_world(tiles: &TileMap, assets: &Assets, player: &Player) {
    let line_length = 10.0 * 8.0;
    let visible_angle = std::f32::consts::PI;
    let ray_amount = 35;
    let angle_increment = visible_angle / ray_amount as f32;

    let angles: Vec<f32> = (0..ray_amount)
        .map(|ray| {
            player.angle
                + angle_increment * ray as f32
                - visible_angle / 2.0
                + angle_increment / 2.0
        })
        .collect();

    let definition = 2.5;
    let visible_tiles = combine_hashsets(
        angles
            .iter()
            .map(|&angle| find_tiles(player.pos, angle, line_length / 8.0, definition))
            .collect(),
    );

    draw_tiles(tiles, assets, visible_tiles);
    for angle in &angles {
        draw_line(
            player.pos.x, 
            player.pos.y,
            player.pos.x + (angle + std::f32::consts::FRAC_PI_2).cos() * line_length,
            player.pos.y + (angle + std::f32::consts::FRAC_PI_2).sin() * line_length,
            1.0,
            RED,
        );
    }
}


#[macroquad::main(conf)]
async fn main() {
    let mut camera = GameCamera::new();
    
    let assets = Assets::new().await;
    
    let world = maps::example_world();

    let mut dummy = Dummy { pos: vec2(2.2 * 8.0, 9.3 * 8.0), angle: 0.0 };

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
        draw_text(
            &get_fps().to_string(),
            50.0,
            70.0, 
            100.0,
            WHITE,
        );
        draw_text(
            match is_windows() {
                true => "windows",
                false => "linux",
            },
            50.0,
            140.0, 
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

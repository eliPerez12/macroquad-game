use assets::Assets;
use camera::GameCamera;
use entities::*;
use macroquad::{miniquad::conf::Icon, prelude::*};
use player::Player;
use ui::{render_debug_ui, render_ui};
use world::{draw_world, draw_collidables};

mod assets;
mod camera;
mod entities;
mod items;
mod maps;
mod player;
mod ui;
mod utils;
mod world;

#[macroquad::main(conf)]
async fn main() {
    let mut camera = GameCamera::new();
    let assets = Assets::new().await;
    let world = maps::example_world();
    let mut dummy = Dummy {
        pos: vec2(2.2 * 8.0, 9.3 * 8.0),
        angle: 0.0,
    };
    let mut debug_on = false;

    let mut player = Player::new();
    player.pos = Vec2::new(60.0, 50.0);
    camera.target = player.pos;
    let mut bullets: Vec<Bullet> = vec![];
    // Main game loop
    loop {
        // Update Game
        player.handle_player_movements(&camera, &world);
        handle_shooting(&mut bullets, &assets, &player, &camera, &world);
        dummy.turn_to_face(&camera, player.pos);
        camera.handle_controls();
        camera.pan_to_target(player.pos);

        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::T) {
            debug_on = !debug_on;
        }

        // Draw in world space
        set_camera(&camera);

        // Draws example world
        draw_world(&world, &assets, &player);

        // Draw player
        player.draw(&assets);
        
        // Draw dummy
        dummy.draw(&assets, &player);

        if debug_on {
            // player._draw_debug_rays();
            player.draw_hitbox();
            draw_collidables(&world);
        }

        // Bullets
        for bullet in &bullets {
            draw_circle(bullet.pos.x, bullet.pos.y, 0.2, WHITE);
        }

        // Draw in screen space
        set_default_camera();

        // Rendering UI
        render_ui(&player);
        if debug_on {
            render_debug_ui(&player, &camera);
        }
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
            small: [0; 1024],
            medium: [0; 4096],
            big: [0; 16384],
        }),
        ..Default::default()
    }
}

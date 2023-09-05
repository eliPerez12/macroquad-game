use assets::Assets;
use camera::GameCamera;
use entities::*;
use macroquad::{miniquad::conf::Icon, prelude::*};
use player::{Player, PlayerController};
use ui::{render_debug_ui, render_ui, FpsBarGraph};
use world::{draw_collidables, draw_world};

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
    let mut fps_graph = FpsBarGraph::new();
    let mut debug_on = false;

    let mut player = Player::new(55, 55);
    let mut enemy = Player::new(47, 47);

    player.tp_grid(55, 55);
    player.controller = PlayerController::User; // Allow control from the user

    enemy.tp_grid(47, 47);

    camera.target = player.pos;

    let mut bullets: Vec<Bullet> = vec![];
    // Main game loop
    loop {
        // Update Game
        player.update(&camera, &world);
        handle_shooting(&mut bullets, &assets, &player, &camera, &world);
        camera.handle_controls();
        camera.pan_to_target(player.pos);

        enemy.turn_to_face(player.pos, &camera);

        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::T) {
            debug_on = !debug_on;
        }

        fps_graph.update();

        // Draw in world space
        set_camera(&camera);

        // Draws example world
        draw_world(&world, &assets, &player, &camera);

        // Draw player
        player.draw(&assets);
        // Draw enemy 
        enemy.draw(&assets);


        // Draw Bullets
        for bullet in &bullets {
            draw_circle(bullet.pos.x, bullet.pos.y, 0.2, WHITE);
        }

        if debug_on {
            player.draw_hitbox();
            enemy.draw_hitbox();
            draw_collidables(&world, &camera);
        }

        // Draw in screen space
        set_default_camera();

        // Rendering UI
        render_ui(&player);
        if debug_on {
            render_debug_ui(&player, &camera, &world);
            fps_graph.draw();
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

use assets::Assets;
use camera::GameCamera;
use macroquad::{miniquad::conf::Icon, prelude::*};
use player::{Player, PlayerController};
use ui::{render_debug_ui, render_ui, FpsBarGraph};
use world::World;

mod assets;
mod camera;
mod entities;
mod items;
mod maps;
mod player;
mod ui;
mod utils;
mod world;
mod tile_map;

#[macroquad::main(conf)]
async fn main() {
    let mut camera = GameCamera::new();
    let assets = Assets::new().await;
    let mut fps_graph =  FpsBarGraph::new();
    let mut player = Player::new(52, 55);
    let mut world = World::new();
    let mut debug_on = false;

    player.controller = PlayerController::User; // Allow control from the user
    camera.target = player.pos; // Teleport camera to player
    world.entities.add_player(Player::new(49, 48)); // Spawn AI player

    // Main game loop
    loop {
        // Update Game
        player.update(&camera, &world.tile_map);
        world.entities.handle_shooting(&assets, &player, &camera, &world.tile_map);
        camera.handle_controls();
        camera.pan_to_target(player.pos);

        world.entities.update(&player, &camera);

        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::T) {
            debug_on = !debug_on;
        }

        fps_graph.update();

        ////// Draw in world space //////
        set_camera(&camera);

        world.draw(&camera, &player, &assets);

        // Draw player
        player.draw(&assets);

        // Draw debug thingys
        if debug_on {
            player.draw_hitbox();
            world.draw_debug(&camera);
        }

        ////// Draw in screen space //////
        set_default_camera();

        // Rendering UI
        render_ui(&player);
        if debug_on {
            render_debug_ui(&player, &camera, &world.tile_map);
            fps_graph.draw();
        }

        next_frame().await;
    }
}

fn conf() -> Conf {
    Conf {
        window_title: String::from("Top down shooter"),
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

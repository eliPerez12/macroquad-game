use assets::Assets;
use camera::GameCamera;
use macroquad:: prelude::*;
use player::*;
use ui::*;
use world::World;
use utils::conf;

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
        world.update(&player, &camera, &assets);
        camera.handle_controls();
        camera.pan_to_target(player.pos);

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
use macroquad::prelude::*;
use assets::Assets;
use camera::GameCamera;
use entities::Grenade;
use player::*;
use ui::*;
use utils::conf;
use world::World;

mod assets;
mod camera;
mod entities;
mod items;
mod maps;
mod player;
mod tile_map;
mod ui;
mod utils;
mod world;

#[macroquad::main(conf)]
async fn main() {
    let mut camera = GameCamera::new();
    let assets = Assets::new().await;
    let mut fps_graph = FpsBarGraph::new();
    let mut player = Player::new(52, 55);
    let mut world = World::new();
    let mut debug_on = false;

    player.controller = PlayerController::User; // Allow control from the user
    camera.target = player.pos; // Teleport camera to player
    world.entities.add_player(Player::new(48, 48)); // Spawn other player

    // Main game loop
    loop {
        player.update(&camera, &world.tile_map);
        world.update(&player, &camera, &assets).await;
        camera.handle_controls();
        camera.pan_to_target(player.pos);

        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::T) {
            debug_on = !debug_on;
        }

        if is_key_pressed(KeyCode::G) {
            world.entities.grenades.push(Grenade {
                pos: camera.screen_to_world(mouse_position().into()),
                fuse_time: Grenade::MAX_FUSE_TIME,
                rotation: 0.0,
                rotation_speed: 0.1,
            });
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

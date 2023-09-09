use assets::Assets;
use camera::GameCamera;
use entities::EntityManager;
use macroquad::{miniquad::conf::Icon, prelude::*};
use player::{Player, PlayerController};
use ui::{render_debug_ui, render_ui, FpsBarGraph};

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
    let (
        mut camera,
        assets,
        tile_map,
        mut fps_graph
    ) = init().await;
    let mut debug_on = false;

    let mut player = Player::new(52, 55);
    player.controller = PlayerController::User; // Allow control from the user

    camera.target = player.pos;

    let mut entity_manager = EntityManager::new();
    entity_manager.add_player(Player::new(49, 48));
    // Main game loop
    loop {
        // Update Game
        player.update(&camera, &tile_map);
        entity_manager.handle_shooting(&assets, &player, &camera, &tile_map);
        camera.handle_controls();
        camera.pan_to_target(player.pos);

        entity_manager.update(&player, &camera);
        
        if is_key_down(KeyCode::LeftControl) && is_key_pressed(KeyCode::T) {
            debug_on = !debug_on;
        }

        fps_graph.update();

        ////// Draw in world space //////
        set_camera(&camera);

        // Draws example world
        tile_map.draw_world( &assets, &player, &camera);

        // Draw player
        player.draw(&assets);

        // Draw entities
        entity_manager.draw_entities(&assets, &player, &tile_map);
        
        // Draw debug thingys
        if debug_on {
            player.draw_hitbox();
            entity_manager.draw_entity_hitboxes();
            tile_map.draw_collidables(&camera);
        }

        ////// Draw in screen space //////
        set_default_camera();

        // Rendering UI
        render_ui(&player);
        if debug_on {
            render_debug_ui(&player, &camera, &tile_map);
            fps_graph.draw();
        }

        next_frame().await;
    }
}

async fn init() -> (GameCamera, Assets, world::TileMap, FpsBarGraph) {
    (
        GameCamera::new(),
        Assets::new().await,
        maps::example_world(),
        FpsBarGraph::new()
    )
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
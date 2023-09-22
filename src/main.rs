use std::f32::consts::{PI, E};

use assets::Assets;
use camera::GameCamera;
use entities::Bullet;
use macroquad::prelude::*;
use maps::TILE_COLLIDER_LOOKUP;
use player::*;
use tile_map::LineSegment;
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
    world.entities.add_player(Player::new(48, 48)); // Spawn AI player

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

        if is_key_pressed(KeyCode::G) {
            for _ in 0..60 {
                world.entities.bullets.push(Bullet {
                    pos: camera.screen_to_world(mouse_position().into()),
                    last_pos: camera.screen_to_world(mouse_position().into()),
                    vel: 3.3 + rand::gen_range(-1.5, 1.5),
                    angle: rand::gen_range(0.0, 2.0 * PI),
                    hit_something: false,
                })
            }
        }

        // Draw debug thingys
        if debug_on {
            player.draw_hitbox();
            world.draw_debug(&camera);
            for i in 0..world.tile_map.data.len() {
                let (grid_x, grid_y) = (
                    i as u16 % world.tile_map.width,
                    i as u16 / world.tile_map.height,
                );
                let tile = world.tile_map.get_tile(grid_x, grid_y);
                if let Some(tile) = tile {
                    if TILE_COLLIDER_LOOKUP[tile.0 as usize - 1] {
                        let rect = Rect {
                            x: grid_x as f32 * 8.0,
                            y: grid_y as f32 * 8.0,
                            w: 8.0,
                            h: 8.0,
                        };

                        let (left, right, top, bottom) =  LineSegment::from_rect(&rect);
                        let color = ORANGE;
                        left.draw(color);
                        right.draw(color);
                        top.draw(color);
                        bottom.draw(color);
                    }
                }
            }
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

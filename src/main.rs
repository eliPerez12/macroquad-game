use assets::Assets;
use camera::GameCamera;
use entities::*;
use macroquad::{miniquad::conf::Icon, prelude::*};
use player::{Player, PlayerController};
use ui::{render_debug_ui, render_ui, FpsBarGraph};
use world::LINE_LENGTH;

mod assets;
mod camera;
mod entities;
mod items;
mod maps;
mod player;
mod ui;
mod utils;
mod world;

pub struct EntityManager {
    pub other_players: Vec<Option<Player>>,
    pub other_player_index: u32
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            other_players: vec![],
            other_player_index: 0,
        }
    }
    pub fn update(&mut self, player: &Player, camera: &GameCamera) {
        for other_player in &mut self.other_players {
            if let Some(other_player) = other_player {
                other_player.turn_to_face(player.pos, &camera);
            }
        }
    }
    pub fn draw_entities(&self, assets: &Assets, player: &Player, tile_map: &world::TileMap) {
        let visible_tiles = tile_map.find_tiles(
            player.get_player_rays(std::f32::consts::PI, LINE_LENGTH),
            LINE_LENGTH / 8.0 * 1.0,
            player.pos,
        );
        for other_player in &self.other_players {
            if let Some(other_player) = other_player {
                let dist_to_player = {
                    let dx = other_player.pos.x - player.pos.x;
                    let dy = other_player.pos.y - player.pos.y;
                    (dx * dx + dy * dy).sqrt()
                };
                if visible_tiles.contains(&((other_player.pos.x / 8.0) as u16,(other_player.pos.y / 8.0) as u16)) ||
                dist_to_player < 18.0 {
                    other_player.draw(&assets);
                }
            }
        }
    }
    fn draw_entity_hitboxes(&self) {
        for other_player in &self.other_players {
            if let Some(other_player) = other_player {
                other_player.draw_hitbox();
            }
        }
    }
    fn add_player(&mut self, player: Player) {
        self.other_players.insert(self.other_player_index as usize, Some(player));
    }
}

#[macroquad::main(conf)]
async fn main() {
    let (mut camera, assets, world, mut fps_graph) = init().await;
    let mut debug_on = false;

    let mut player = Player::new(52, 55);
    player.controller = PlayerController::User; // Allow control from the user

    camera.target = player.pos;

    let mut bullets: Vec<Bullet> = vec![];

    let mut entity_manager = EntityManager::new();
    entity_manager.add_player(Player::new(49, 48));
    // Main game loop
    loop {
        // Update Game
        player.update(&camera, &world);
        handle_shooting(&mut bullets, &assets, &player, &camera, &world);
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
        world.draw_world( &assets, &player, &camera);

        // Draw player
        player.draw(&assets);

        // Draw enemy
        entity_manager.draw_entities(&assets, &player, &world);
        
        // Draw Bullets
        for bullet in &bullets {
            draw_circle(bullet.pos.x, bullet.pos.y, 0.2, WHITE);
        }

        if debug_on {
            player.draw_hitbox();
            entity_manager.draw_entity_hitboxes();
            world.draw_collidables(&camera);
        }

        ////// Draw in screen space //////
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
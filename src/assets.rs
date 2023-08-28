use image::{DynamicImage, GenericImageView};
use macroquad::{
    audio::{load_sound, Sound},
    prelude::*,
};
use std::collections::HashMap;
use crate::utils::is_windows;

pub struct Assets {
    textures: HashMap<String, Texture2D>,
    sounds: HashMap<String, Sound>,
}

impl Assets {
    pub fn get_texture(&self, texture: &str) -> &Texture2D {
        self.textures.get(texture).unwrap()
    }

    pub fn get_sound(&self, sound: &str) -> &Sound {
        self.sounds.get(sound).unwrap()
    }

    async fn load_texture(path: &str) -> Result<Texture2D, macroquad::Error> {
        let texture = Texture2D::from_image(&load_image(path).await?);
        texture.set_filter(FilterMode::Nearest);
        Ok(texture)
    }

    // Stack based search through assets folder, and loads in all assets
    pub async fn load_all_assets() -> Self {
        let mut textures: HashMap<String, Texture2D> = HashMap::new();
        let mut sounds: HashMap<String, Sound> = HashMap::new();
        let mut dirs_to_explore = vec![std::path::PathBuf::from("assets")];

        while let Some(dir) = dirs_to_explore.pop() {
            for entry in std::fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() && path.to_str().unwrap() != "temp" {
                    dirs_to_explore.push(path);
                } else if path.is_file() {
                    let path_str = path.to_string_lossy().to_string();
                    // Images
                    if path_str.ends_with(".png") {
                        let key_path_str = match is_windows() {
                            true => path_str.split("\\").last().unwrap(),
                            false => path_str.split("/").last().unwrap(),
                        } ;
                        textures.insert(
                            key_path_str.to_string(),
                            Assets::load_texture(&path_str).await.unwrap(),
                        );
                    }
                    // Sounds
                    if path_str.ends_with(".wav") {
                        let key_path_str = match is_windows() {
                            true => path_str.split("\\").last().unwrap(),
                            false => path_str.split("/").last().unwrap(),
                        } ;
                        let sound = load_sound(&path_str).await.unwrap();
                        sounds.insert(key_path_str.to_string(), sound);
                    }
                }
            }
        }
        Assets { textures, sounds }
    }

    pub async fn new() -> Self {
        Self::load_all_assets().await
    }

    pub async fn get_clothes_from_bitmap(dyn_image: &DynamicImage, clothes: PlayerClothesNormal) -> Texture2D {
    
        let mut image = Image::empty();
        image.width = dyn_image.width() as u16;
        image.height = dyn_image.height() as u16;
        image.bytes = vec![0;(image.width * image.height * 4) as usize];
        for pixel in dyn_image.pixels() {
            if pixel.2.0[3] == 0 {continue};
            let colors = clothes.get_colors();
            let color = match pixel.2.0[0] {
                67 => colors.0,
                47 => colors.1,
                64 => colors.2,
                35 => colors.3,
                27 => colors.4,
                _ => Color::new(
                    pixel.2.0[0] as f32 / 255.0,
                    pixel.2.0[1] as f32 / 255.0,
                    pixel.2.0[2] as f32 / 255.0,
                    pixel.2.0[3] as f32 / 255.0,
                ),
            };
            image.set_pixel(pixel.0, pixel.1, color);
        };
    
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);
    
        texture
    }
}

pub enum PlayerClothesNormal {
    Blue,
    Dark,
}

impl PlayerClothesNormal {
    fn get_colors(&self) -> (Color, Color, Color, Color, Color) {
        match self {
            PlayerClothesNormal::Blue => (
                Color::from_rgba(41, 98, 173, 255),
                Color::from_rgba(56, 61, 115, 255),
                Color::from_rgba(49, 86, 135, 255),
                Color::from_rgba(38, 41, 79, 255),
                Color::from_rgba(25, 27, 48, 255),
            ),
            PlayerClothesNormal::Dark => (
                Color::from_rgba(67, 67, 67, 255),
                Color::from_rgba(47, 47, 47, 255),
                Color::from_rgba(64, 64, 64, 255),
                Color::from_rgba(35, 35, 35, 255),
                Color::from_rgba(27, 27, 27, 255),
            ),
        }
    }
}
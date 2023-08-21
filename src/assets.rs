use macroquad::prelude::*;
use std::collections::HashMap;

pub struct Assets {
    textures: HashMap<String, Texture2D>,
}

impl Assets {
    pub fn get_texture(&self, texture: &str) -> &Texture2D{
        self.textures.get(texture).unwrap()
    }

    async fn load_texture(path: &str) -> Result<Texture2D, macroquad::Error> {
        let texture = Texture2D::from_image(&load_image(path).await?);
        texture.set_filter(FilterMode::Nearest);
        Ok(texture)
    }

    // Stack based search through assets folder, and loads in all assets
    pub async fn load_all_assets() -> Self {
        let mut textures: HashMap<String, Texture2D> = HashMap::new();
        let mut dirs_to_explore = vec![std::path::PathBuf::from("assets")];
    
        while let Some(dir) = dirs_to_explore.pop() {
            for entry in std::fs::read_dir(dir).unwrap(){
                let entry = entry.unwrap();
                let path = entry.path();
                if path.is_dir() && path.to_str().unwrap() != "temp"{
                    dirs_to_explore.push(path);
                } else if path.is_file() {
                    let path_str = path.to_string_lossy().to_string();
                    if path_str.ends_with(".png") {
                        let key_path_str = path_str.split("/").last().unwrap();
                        textures.insert(key_path_str.to_string(), Assets::load_texture(&path_str).await.unwrap());
                    }
                }
            }
        }
        Assets { textures }
    }
}

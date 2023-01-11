pub mod loadable;
pub mod meshmanager;
pub mod model;
pub mod texmanager;
pub mod texture;

use self::{
    loadable::*, meshmanager::MeshManager, model::ModelData, texmanager::TexManager,
    texture::TextureData,
};
use std::{collections::HashMap, num::NonZeroU32};

use super::renderer::RendererState;

pub struct ResourceManager {
    pub models: HashMap<String, ResourceBox<TextureData>>,
    pub textures: HashMap<String, ResourceBox<ModelData>>,
    pub mesh_manager: MeshManager,
    pub tex_manager: TexManager,
}

impl ResourceManager {
    pub fn new(root: &'static str, renderer: &RendererState) -> ResourceManager {
        let mut manager = Self {
            models: HashMap::new(),
            textures: HashMap::new(),
            mesh_manager: MeshManager::new(&renderer.device, 4096),
            tex_manager: TexManager::new(&renderer.device, NonZeroU32::new(256).unwrap()),
        };

        manager.read_files(root, root);
        manager
    }

    fn read_files(&mut self, root: &str, path: &str) {
        if let Ok(dir_iter) = std::fs::read_dir(path) {
            for dir_entry in dir_iter {
                if let Ok(entry) = dir_entry {
                    let entry_path = entry.path();
                    if entry_path.is_dir() {
                        self.read_files(root, path);
                    } else {
                        let rel = pathdiff::diff_paths(&entry_path, root)
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_string();
                        if let Some(ext) = entry_path.extension() {
                            match ext.to_str() {
                                Some("obj") => {
                                    self.models.insert(rel, ResourceBox::new(path));
                                }
                                Some("png") => {
                                    self.textures.insert(rel, ResourceBox::new(path));
                                }
                                _ => (),
                            }
                        }
                    }
                }
            }
        }
    }
}

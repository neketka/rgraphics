mod loadable;
mod meshmanager;
mod model;
mod texture;

use self::{loadable::*, meshmanager::MeshManager, model::ModelData, texture::TextureData};
use std::{collections::HashMap, rc::Rc};

use super::renderer::Renderer;

pub struct ResourceManager<'a> {
    models: HashMap<String, ResourceBox<'a, TextureData>>,
    textures: HashMap<String, ResourceBox<'a, ModelData>>,
    mesh_manager: MeshManager<'a>,
}

impl<'a> ResourceManager<'a> {
    pub fn new(root: &'a str, renderer: Rc<Renderer>) -> ResourceManager<'a> {
        let mut manager = Self {
            models: HashMap::new(),
            textures: HashMap::new(),
            mesh_manager: MeshManager::new(renderer, 1024),
        };

        manager.read_files(root, root);
        manager
    }

    fn read_files(&mut self, root: &str, path: &'a str) {
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

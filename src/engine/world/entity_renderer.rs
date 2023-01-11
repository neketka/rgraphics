use std::collections::HashMap;

use cgmath::Matrix4;

use crate::engine::{
    resource::{model::ModelData, texture::TextureData},
    EngineResources,
};

use super::RenderId;

pub enum RenderTargetType {
    GBuffer,
    RGBADepth,
    RGBA,
    Depth,
}

pub enum EntityTexture {
    Resource(&'static str),
    Dynamic(TextureData),
    RenderTarget {
        width: i32,
        height: i32,
        ty: RenderTargetType,
        post_enabled: bool,
    },
}

pub enum EntityModel {
    Resource(&'static str),
    Dynamic(ModelData),
}

pub struct RenderEntity {
    textures: Vec<EntityTexture>,
    model: Option<EntityModel>,
}

pub struct EntityRendererStorage {
    renders: HashMap<RenderId, RenderEntity>,
    id_counter: u64,
}

impl EntityRendererStorage {
    pub fn new() -> EntityRendererStorage {
        EntityRendererStorage {
            renders: HashMap::new(),
            id_counter: 0,
        }
    }
}

pub struct RenderTask(RenderId, Vec<Matrix4<f32>>);

pub struct EntityRenderer<'a> {
    pub storage: &'a mut EntityRendererStorage,
    pub resources: &'a mut EngineResources,
}

impl EntityRenderer<'_> {
    pub fn create_render(
        &mut self,
        textures: Vec<EntityTexture>,
        model: Option<EntityModel>,
    ) -> RenderId {
        self.storage
            .renders
            .insert(self.storage.id_counter, RenderEntity { textures, model });

        let handle = self.storage.id_counter;
        self.storage.id_counter += 1;
        handle
    }

    pub fn delete_render(&mut self, id: RenderId) {}

    pub fn set_render_vertices(&mut self, index: usize, data: EntityModel, fit: bool) {}

    pub fn set_render_texture(&mut self, index: usize, data: EntityTexture) {}

    pub fn render(
        &mut self,
        target: RenderId,
        rt_index: i32,
        camera: &Matrix4<f32>,
        tasks: &[RenderTask],
    ) {
    }
}

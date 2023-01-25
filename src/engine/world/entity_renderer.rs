use std::collections::HashMap;

use cgmath::Matrix4;

use crate::engine::{
    resource::{
        meshmanager::MeshHandle,
        model::ModelVertex,
        texmanager::{TexDataFormat, TexHandle},
    },
    EngineResources,
};

use super::RenderId;

pub enum SetVerticesData<'a> {
    Replace(&'a [ModelVertex]),
    ModifyAt(usize, &'a [ModelVertex]),
}

pub enum RenderTargetType {
    RGBA32Depth,
    RGBA32,
    Depth,
}

pub enum EntityTexture {
    Resource(Vec<&'static str>),
    DynamicRGBA(Vec<(u32, u32)>),
    RenderTarget {
        width: u32,
        height: u32,
        ty: RenderTargetType,
        post_enabled: bool,
    },
}

pub enum EntityModel {
    Resource(&'static str),
    InitialSize(usize),
    Alias(RenderId),
}

pub struct RenderEntity {
    texture: Option<EntityTexture>,
    model: Option<EntityModel>,
    color_allocations: Vec<TexHandle>,
    depth_allocation: Option<TexHandle>,
    mesh_allocation: Option<MeshHandle>,
}

pub struct EntityRendererStorage {
    renders: HashMap<RenderId, RenderEntity>,
    id_counter: u64,
}

impl EntityRendererStorage {
    pub fn new() -> EntityRendererStorage {
        Self {
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
        texture: Option<EntityTexture>,
        model: Option<EntityModel>,
    ) -> RenderId {
        let mut colors = Vec::new();
        let mut depth = None;
        let mut mesh = None;

        match texture {
            Some(EntityTexture::Resource(ref paths)) => {
                for path in paths.iter() {
                    let tex = self
                        .resources
                        .resource_manager
                        .textures
                        .get_mut(*path)
                        .unwrap();

                    colors.push(self.resources.resource_manager.tex_manager.alloc_tex(
                        &self.resources.renderer.device,
                        &self.resources.renderer.queue,
                        TexDataFormat::StaticRGBA8(tex.load().unwrap().as_ref()),
                    ));
                }
            }
            Some(EntityTexture::DynamicRGBA(ref sizes)) => {
                for (w, h) in sizes.iter() {
                    colors.push(self.resources.resource_manager.tex_manager.alloc_tex(
                        &self.resources.renderer.device,
                        &self.resources.renderer.queue,
                        TexDataFormat::DynamicRGBA32(*w, *h),
                    ));
                }
            }
            Some(EntityTexture::RenderTarget {
                width,
                height,
                ref ty,
                post_enabled,
            }) => {
                if let RenderTargetType::RGBA32Depth | RenderTargetType::RGBA32 = ty {
                    colors.push(self.resources.resource_manager.tex_manager.alloc_tex(
                        &self.resources.renderer.device,
                        &self.resources.renderer.queue,
                        TexDataFormat::DynamicRGBA32(width, height),
                    ));

                    if post_enabled {
                        colors.push(self.resources.resource_manager.tex_manager.alloc_tex(
                            &self.resources.renderer.device,
                            &self.resources.renderer.queue,
                            TexDataFormat::DynamicRGBA32(width, height),
                        ));
                    }
                }

                if let RenderTargetType::RGBA32Depth | RenderTargetType::Depth = ty {
                    depth = Some(self.resources.resource_manager.tex_manager.alloc_tex(
                        &self.resources.renderer.device,
                        &self.resources.renderer.queue,
                        TexDataFormat::DynamicDepth(width, height),
                    ));
                }
            }
            None => {}
        };

        match &model {
            Some(EntityModel::Resource(path)) => {
                let model = self
                    .resources
                    .resource_manager
                    .models
                    .get_mut(*path)
                    .expect(format!("Cannot find model at '{path}'").as_str())
                    .load()
                    .unwrap();

                mesh = Some(
                    self.resources
                        .resource_manager
                        .mesh_manager
                        .alloc_mesh(model.vertices.len()),
                );
            }
            Some(EntityModel::InitialSize(init_size)) => {
                mesh = Some(
                    self.resources
                        .resource_manager
                        .mesh_manager
                        .alloc_mesh(*init_size),
                );
            }
            Some(EntityModel::Alias(_)) => {}
            None => {}
        }

        self.storage.renders.insert(
            self.storage.id_counter,
            RenderEntity {
                texture,
                model,
                color_allocations: colors,
                depth_allocation: depth,
                mesh_allocation: mesh,
            },
        );

        let handle = self.storage.id_counter;
        self.storage.id_counter += 1;
        handle
    }

    pub fn delete_render(&mut self, id: RenderId) {
        let entity = self.storage.renders.remove(&id).unwrap();

        for alloc in entity.color_allocations {
            self.resources.resource_manager.tex_manager.free_tex(alloc);
        }

        if let Some(alloc) = entity.depth_allocation {
            self.resources.resource_manager.tex_manager.free_tex(alloc);
        }

        if let Some(alloc) = entity.mesh_allocation {
            self.resources
                .resource_manager
                .mesh_manager
                .free_mesh(alloc);
        }
    }

    pub fn set_vertices(&mut self, id: RenderId, data: SetVerticesData) {
        let mut render = self.storage.renders.get_mut(&id).unwrap();

        let buf_size = self
            .resources
            .resource_manager
            .mesh_manager
            .get_range(render.mesh_allocation.as_ref().unwrap())
            .len();

        match data {
            SetVerticesData::Replace(slice) => {
                if slice.len() != buf_size {
                    let old_handle = render.mesh_allocation.take().unwrap();

                    self.resources
                        .resource_manager
                        .mesh_manager
                        .free_mesh(old_handle);

                    render.mesh_allocation = Some(
                        self.resources
                            .resource_manager
                            .mesh_manager
                            .alloc_mesh(slice.len()),
                    );
                }

                self.resources.resource_manager.mesh_manager.set_vertices(
                    &self.resources.renderer.queue,
                    slice,
                    render.mesh_allocation.as_ref().unwrap(),
                    0,
                );
            }
            SetVerticesData::ModifyAt(index, slice) => {
                let alloc_size = index as usize + slice.len();

                if alloc_size > buf_size {
                    panic!("Mesh size in EntityRender is too small");
                }

                self.resources.resource_manager.mesh_manager.set_vertices(
                    &self.resources.renderer.queue,
                    slice,
                    render.mesh_allocation.as_ref().unwrap(),
                    index,
                );
            }
        }
    }

    //pub fn set_texture(&mut self, id: RenderId, index: usize, data: &[u8]) {}

    pub fn render(&mut self, target: RenderId, camera: &Matrix4<f32>, tasks: &[RenderTask]) {}
}

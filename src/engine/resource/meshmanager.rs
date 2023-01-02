use std::{collections::HashMap, rc::Rc};

use wgpu::*;

use super::model::ModelVertex;
use crate::engine::renderer::Renderer;

struct MeshAllocation {
    first_vertex: i32,
    count: i32,
}

pub struct MeshHandle(i32);

pub struct MeshManager<'a> {
    pub vertex_buffer: Buffer,
    pub vertex_layout: VertexBufferLayout<'a>,
    vertex_count: i32,
    allocations: HashMap<i32, MeshAllocation>,
    free_indices: Vec<i32>,
    min_free_id: i32,
    renderer: Rc<Renderer>,
}

impl<'a> MeshManager<'a> {
    pub fn new(renderer: Rc<Renderer>, size: i32) -> Self {
        let buf = renderer.device.create_buffer(&BufferDescriptor {
            label: Some("MeshManager Vertices"),
            size: (std::mem::size_of::<ModelVertex>() * size as usize) as u64,
            usage: BufferUsages::COPY_SRC | BufferUsages::COPY_DST | BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        let layout = VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelVertex>() as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &[
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: 0,
                    shader_location: 0,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: (std::mem::size_of::<f32>() * 3) as u64,
                    shader_location: 1,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x3,
                    offset: (std::mem::size_of::<f32>() * 6) as u64,
                    shader_location: 2,
                },
            ],
        };

        Self {
            renderer,
            vertex_buffer: buf,
            vertex_layout: layout,
            vertex_count: size,
            min_free_id: 1,
            free_indices: vec![0],
            allocations: HashMap::from([(
                0,
                MeshAllocation {
                    first_vertex: 0,
                    count: size,
                },
            )]),
        }
    }

    pub fn alloc_mesh(&mut self, size: i32) -> MeshHandle {
        for index in 0..self.allocations.len() {
            let free_id = self.free_indices[index];
            let alloc = self.allocations.get(&(index as i32)).unwrap();
            if alloc.count == size {
                self.free_indices.remove(free_id as usize);
                return MeshHandle(free_id);
            } else if alloc.count > size {
                self.free_indices.remove(free_id as usize);
                self.free_indices.push(self.min_free_id);
                self.allocations.insert(
                    self.min_free_id,
                    MeshAllocation {
                        first_vertex: alloc.first_vertex + size,
                        count: alloc.count - size,
                    },
                );
                return MeshHandle(free_id);
            }
        }
        panic!("Unable to find allocation for mesh!");
    }

    pub fn free_mesh(&mut self, handle: MeshHandle) {
        self.free_indices.push(handle.0);
    }

    pub fn set_vertices(&mut self, data: &[ModelVertex], offset: i32) {
        self.renderer.queue.write_buffer(
            &self.vertex_buffer,
            offset as u64,
            bytemuck::cast_slice(data),
        );
    }
}

use std::{collections::HashMap, ops::Range};

use wgpu::*;

use super::model::ModelVertex;

struct MeshAllocation {
    first_vertex: usize,
    count: usize,
}

#[derive(Debug)]
pub struct MeshHandle(i32);

pub struct MeshManager {
    pub vertex_buffer: Buffer,
    pub vertex_layout: VertexBufferLayout<'static>,
    vertex_count: usize,
    allocations: HashMap<i32, MeshAllocation>,
    free_indices: Vec<i32>,
    min_free_id: i32,
}

static ATTR_ARR: [VertexAttribute; 3] =
    vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x2];

impl MeshManager {
    pub fn new(device: &Device, size: usize) -> Self {
        let buf = device.create_buffer(&BufferDescriptor {
            label: Some("MeshManager Vertices"),
            size: (std::mem::size_of::<ModelVertex>() * size as usize) as u64,
            usage: BufferUsages::COPY_SRC | BufferUsages::COPY_DST | BufferUsages::VERTEX,
            mapped_at_creation: false,
        });

        let layout = VertexBufferLayout {
            array_stride: std::mem::size_of::<ModelVertex>() as u64,
            step_mode: VertexStepMode::Vertex,
            attributes: &ATTR_ARR,
        };

        Self {
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

    pub fn alloc_mesh(&mut self, size: usize) -> MeshHandle {
        for index in 0..self.allocations.len() {
            let free_id = self.free_indices[index];
            let alloc = self.allocations.get(&(index as i32)).unwrap();
            if alloc.count == size {
                self.free_indices.remove(free_id as usize);
                return MeshHandle(free_id);
            } else if alloc.count > size {
                self.free_indices.remove(free_id as usize);
                self.free_indices.push(self.min_free_id);
                self.min_free_id += 1;
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

    pub fn set_vertices(
        &self,
        queue: &Queue,
        data: &[ModelVertex],
        handle: &MeshHandle,
        offset: usize,
    ) {
        let alloc = self.allocations.get(&handle.0).unwrap();

        queue.write_buffer(
            &self.vertex_buffer,
            (offset + alloc.first_vertex) as u64,
            bytemuck::cast_slice(data),
        );
    }

    pub fn get_range(&self, handle: &MeshHandle) -> Range<usize> {
        let range = self.allocations.get(&handle.0).unwrap();
        range.first_vertex..range.first_vertex + range.count
    }
}

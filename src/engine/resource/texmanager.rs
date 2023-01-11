use std::{collections::HashMap, num::NonZeroU32};

use wgpu::{util::DeviceExt, *};

use super::texture::TextureData;

pub struct TexHandle(u64);

pub struct TexManager {
    committed: Vec<(Texture, TextureView)>,
    mapping_ids: Vec<u64>,
    pub bind_group_layout: BindGroupLayout,
    pub bind_group: BindGroup,
    max_size: usize,
    alloc_mapping: HashMap<u64, usize>,
    id_count: u64,
    bilinear: Sampler,
    nearest: Sampler,
}

impl TexManager {
    pub fn new(device: &Device, size: NonZeroU32) -> TexManager {
        let bilinear = device.create_sampler(&SamplerDescriptor {
            label: Some("TexManager Bilinear"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            lod_max_clamp: 0.0,
            lod_min_clamp: 0.0,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
        });

        let nearest = device.create_sampler(&SamplerDescriptor {
            label: Some("TexManager Nearest"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Linear,
            lod_max_clamp: 0.0,
            lod_min_clamp: 0.0,
            compare: None,
            anisotropy_clamp: None,
            border_color: None,
        });

        let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("TexManager Layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: Some(size),
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        });

        let group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("TexManager Group"),
            layout: &layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureViewArray(&[]),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&bilinear),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(&nearest),
                },
            ],
        });

        TexManager {
            committed: Vec::new(),
            bind_group_layout: layout,
            bind_group: group,
            alloc_mapping: HashMap::new(),
            id_count: 0,
            bilinear,
            nearest,
            mapping_ids: Vec::new(),
            max_size: size.get() as usize,
        }
    }

    pub fn alloc_tex(
        &mut self,
        device: &Device,
        queue: &Queue,
        tex_data: &TextureData,
    ) -> TexHandle {
        if self.max_size == self.committed.len() {
            panic!("Unable to allocate texture!");
        }

        let desc = TextureDescriptor {
            label: None,
            size: Extent3d {
                width: tex_data.width as u32,
                height: tex_data.height as u32,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            usage: TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
        };

        let tex = device.create_texture_with_data(queue, &desc, tex_data.data.as_slice());
        let view = tex.create_view(&TextureViewDescriptor::default());

        self.committed.push((tex, view));
        self.mapping_ids.push(self.id_count);

        self.rebuild_binds(device);

        let handle = TexHandle(self.id_count);

        self.alloc_mapping
            .insert(self.id_count, self.committed.len() - 1);

        self.id_count += 1;

        handle
    }

    fn free_tex(&mut self, device: &Device, alloc: TexHandle) {
        let id = alloc.0;
        let idx = *self.alloc_mapping.get(&id).unwrap();
        let remap_id = self.mapping_ids.swap_remove(idx);

        self.committed.swap_remove(idx);

        self.alloc_mapping.remove(&id);
        self.alloc_mapping.insert(remap_id, idx);
    }

    fn set_data(&mut self, queue: &Queue, alloc: &TexHandle, tex_data: &TextureData) {
        let idx = *self.alloc_mapping.get(&alloc.0).unwrap();
        let tex = self.committed.get(idx).unwrap();

        queue.write_texture(
            ImageCopyTexture {
                texture: &tex.0,
                mip_level: 0,
                origin: Origin3d::ZERO,
                aspect: TextureAspect::All,
            },
            &tex_data.data,
            ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(4 * tex_data.width as u32),
                rows_per_image: NonZeroU32::new(tex_data.height as u32),
            },
            Extent3d {
                width: tex_data.width as u32,
                height: tex_data.height as u32,
                depth_or_array_layers: 1,
            },
        )
    }

    fn get_index(&self, handle: &TexHandle) -> usize {
        *self.alloc_mapping.get(&handle.0).unwrap()
    }

    fn rebuild_binds(&mut self, device: &Device) {
        let group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("TexManager Group"),
            layout: &self.bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureViewArray(
                        self.committed
                            .iter()
                            .map(|view| &view.1)
                            .collect::<Vec<_>>()
                            .as_slice(),
                    ),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&self.bilinear),
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Sampler(&self.nearest),
                },
            ],
        });

        self.bind_group = group;
    }
}

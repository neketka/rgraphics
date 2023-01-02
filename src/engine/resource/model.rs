use super::loadable::Loadable;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModelVertex {
    pos: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
}

unsafe impl bytemuck::Pod for ModelVertex {}
unsafe impl bytemuck::Zeroable for ModelVertex {}

pub struct ModelData {
    vertices: Vec<ModelVertex>,
}

impl Loadable<ModelData> for ModelData {
    fn load(path: &str) -> Result<ModelData, std::io::Error> {
        let mut data = Self {
            vertices: Vec::new(),
        };

        let obj = obj::Obj::load(path).unwrap();
        for object in obj.data.objects {
            for group in object.groups {
                for poly in group.polys {
                    for tri in poly.0 {
                        let indices = [tri.0, tri.1.unwrap(), tri.2.unwrap()];

                        for i in indices {
                            let [x, y, z] = obj.data.position[i];
                            let [u, v] = obj.data.texture[i];
                            let [xn, yn, zn] = obj.data.normal[i];

                            data.vertices.push(ModelVertex {
                                pos: [x, y, z],
                                normal: [xn, yn, zn],
                                uv: [u, v],
                            });
                        }
                    }
                }
            }
        }

        Ok(data)
    }
}

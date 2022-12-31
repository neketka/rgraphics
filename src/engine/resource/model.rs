use cgmath::{vec4, Vector4};

use super::loadable::Loadable;

pub struct ModelVertex {
    pos: Vector4<f32>,
    normal: Vector4<f32>,
    uv: Vector4<f32>,
}
pub struct ModelData {
    vertices: Vec<ModelVertex>,
    indices: Vec<i32>,
}

impl Loadable<ModelData> for ModelData {
    fn load(path: &str) -> Result<ModelData, std::io::Error> {
        let mut data = Self {
            vertices: Vec::new(),
            indices: Vec::new(),
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
                                pos: vec4(x, y, z, 1.0),
                                normal: vec4(xn, yn, zn, 1.0),
                                uv: vec4(u, v, 1.0, 1.0),
                            });
                        }
                    }
                }
            }
        }

        data.indices.extend(0..(data.vertices.len() as i32));

        Ok(data)
    }
}

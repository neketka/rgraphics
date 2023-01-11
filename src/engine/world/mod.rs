pub mod entity_renderer;

use self::entity_renderer::{EntityRenderer, EntityRendererStorage};
use super::EngineResources;
use cgmath::{vec3, Matrix4, Rad, Vector3};
use std::collections::HashMap;

pub type EntityId = u64;
pub type RenderId = u64;

struct EntityData {
    transform: EntityTransform,
    render: Option<RenderId>,
    visible: bool,
}

pub trait WorldBehavior {
    fn init(&mut self, world: &mut World, renderer: &mut EntityRenderer);
    fn run(&mut self, world: &mut World, renderer: &mut EntityRenderer, dt: f32);
}

pub struct EntityTransform {
    pub pos: Vector3<f32>,
    pub rot: Vector3<f32>,
    pub scale: Vector3<f32>,
}

impl EntityTransform {
    pub fn make_model_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_nonuniform_scale(self.scale.x, self.scale.y, self.scale.z)
            * Matrix4::from_angle_z(Rad(self.rot.z))
            * Matrix4::from_angle_x(Rad(self.rot.x))
            * Matrix4::from_angle_y(Rad(self.rot.y))
            * Matrix4::from_translation(self.pos)
    }

    pub fn make_view_matrix(&self) -> Matrix4<f32> {
        Matrix4::from_angle_z(Rad(-self.rot.z))
            * Matrix4::from_angle_x(Rad(-self.rot.x))
            * Matrix4::from_angle_y(Rad(-self.rot.y))
            * Matrix4::from_translation(-self.pos)
    }
}

pub struct World {
    id_counter: u64,

    behaviors: Option<Box<Vec<Box<dyn WorldBehavior>>>>,
    renderer_storage: Option<Box<EntityRendererStorage>>,

    render_to_entities: HashMap<RenderId, Vec<EntityId>>,

    entities: HashMap<EntityId, EntityData>,
    tag_to_entities: HashMap<String, Vec<EntityId>>,
    entity_to_tags: HashMap<EntityId, Vec<String>>,
}

impl World {
    pub fn new(behaviors: Vec<Box<dyn WorldBehavior>>) -> World {
        World {
            id_counter: 0,
            behaviors: Some(Box::new(behaviors)),
            renderer_storage: Some(Box::new(EntityRendererStorage::new())),
            render_to_entities: HashMap::new(),
            entities: HashMap::new(),
            tag_to_entities: HashMap::new(),
            entity_to_tags: HashMap::new(),
        }
    }

    pub fn create_entity(&mut self, tags: &[&str], render: Option<RenderId>) -> EntityId {
        let id = self.id_counter;
        self.id_counter += 1;

        self.entities.insert(
            id,
            EntityData {
                transform: EntityTransform {
                    pos: vec3(0.0, 0.0, 0.0),
                    rot: vec3(0.0, 0.0, 0.0),
                    scale: vec3(1.0, 1.0, 1.0),
                },
                render,
                visible: true,
            },
        );

        let mut tag_vec = Vec::new();
        for &tag in tags {
            tag_vec.push(String::from(tag));
            if !self.tag_to_entities.contains_key(tag) {
                self.tag_to_entities.insert(tag.to_string(), Vec::new());
            }
            self.tag_to_entities.get_mut(tag).unwrap().push(id);
        }

        self.entity_to_tags.insert(id, tag_vec);

        if let Some(render_id) = render {
            self.render_to_entities
                .get_mut(&render_id)
                .unwrap()
                .push(id);
        }
        id
    }

    pub fn delete_entity(&mut self, id: EntityId) {
        let entity = self.entities.remove(&id).unwrap();
        if let Some(render_id) = entity.render {
            let renders = self.render_to_entities.get_mut(&render_id).unwrap();
            let rm_idx = renders.iter().position(|&ent| ent == id).unwrap();
            renders.swap_remove(rm_idx);
        }

        let tags = self.entity_to_tags.remove(&id).unwrap();
        for tag in tags.iter() {
            let tag_vec = self.tag_to_entities.get_mut(tag).unwrap();
            let rm_idx = tag_vec.iter().position(|&ent| ent == id).unwrap();
            tag_vec.swap_remove(rm_idx);
        }
    }

    pub fn get_render(&self, id: EntityId) -> Option<RenderId> {
        self.entities.get(&id).unwrap().render
    }

    pub fn ids_by_tag<'a>(&self, tag: &str) -> &Vec<EntityId> {
        self.tag_to_entities.get(tag).unwrap()
    }

    pub fn tags_by_id<'a>(&self, id: EntityId) -> &Vec<String> {
        self.entity_to_tags.get(&id).unwrap()
    }

    pub fn transform_by_id<'a>(&mut self, id: EntityId) -> &mut EntityTransform {
        &mut self.entities.get_mut(&id).unwrap().transform
    }

    pub fn init(&mut self, resources: &mut EngineResources) {
        let mut behaviors = self.behaviors.take().unwrap();
        let mut storage = self.renderer_storage.take().unwrap();

        let mut renderer = EntityRenderer {
            storage: &mut storage,
            resources,
        };

        for behavior in behaviors.iter_mut() {
            behavior.init(self, &mut renderer);
        }

        self.renderer_storage = Some(storage);
        self.behaviors = Some(behaviors);
    }

    pub fn run(&mut self, dt: f32, resources: &mut EngineResources) {
        let mut behaviors = self.behaviors.take().unwrap();
        let mut storage = self.renderer_storage.take().unwrap();

        let mut renderer = EntityRenderer {
            storage: &mut storage,
            resources,
        };

        for behavior in behaviors.iter_mut() {
            behavior.run(self, &mut renderer, dt);
        }

        self.renderer_storage = Some(storage);
        self.behaviors = Some(behaviors);
    }
}

use nalgebra_glm as glm;

slotmap::new_key_type! {
    pub struct RSGTransformKey;
}

#[derive(Clone, Copy)]
pub struct RSGTransformComponent {
    pub local_transform: glm::Mat4,
    pub world_transform: glm::Mat4
}

impl RSGTransformComponent {
    pub fn new(local_transform: glm::Mat4) -> Self {
        RSGTransformComponent {
            local_transform: local_transform,
            world_transform: local_transform
        }
    }
}

pub type RSGTransformComponentList = slotmap::SlotMap<RSGTransformKey, RSGTransformComponent>;

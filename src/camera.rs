use nalgebra_glm as glm;

slotmap::new_key_type! {
    pub struct RSGCameraKey;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RSGOrthographicProjection {
    pub xmag: f32,
    pub ymag: f32,
    pub near: f32,
    pub far: f32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RSGPerspectiveProjection {
    pub aspect_ratio: f32,
    pub fov: f32,
    pub near: f32,
    pub far: f32
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSGCamera {
    Orthographic(RSGOrthographicProjection),
    Perspective(RSGPerspectiveProjection)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RSGCameraWorldTransformDerivedProperties {
    pub position: glm::Vec3,
    pub direction: glm::Vec3
}

#[derive(Clone, Copy)]
pub struct RSGCameraComponent {
    pub camera: RSGCamera,
    pub world_properties: RSGCameraWorldTransformDerivedProperties
}

impl RSGCameraComponent {
    pub fn new(camera: RSGCamera) -> Self {
        RSGCameraComponent {
            camera: camera,
            world_properties: RSGCameraWorldTransformDerivedProperties {
                position: glm::vec3(0.0, 0.0, 0.0),
                direction: glm::vec3(0.0, 0.0, -1.0)
            }
        }
    }
}

pub type RSGCameraComponentList = slotmap::SlotMap<RSGCameraKey, RSGCameraComponent>;

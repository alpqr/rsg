use crate::scene::RSGNodeKey;
use nalgebra_glm as glm;

slotmap::new_key_type! {
    pub struct RSGMeshKey;
}

#[derive(Clone, Copy)]
pub struct RSGMeshComponent {
    pub sorting_distance: f32,
    pub viewport_node_key: Option<RSGNodeKey>
}

impl RSGMeshComponent {
    pub fn new() -> Self {
        RSGMeshComponent {
            sorting_distance: 0.0,
            viewport_node_key: None
        }
    }
}

pub type RSGMeshComponentList = slotmap::SlotMap<RSGMeshKey, RSGMeshComponent>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSGMeshVertexInputType {
    Float,
    Vec2,
    Vec3,
    Vec4,
    Int,
    Int2,
    Int3,
    Int4,
    Mat2,
    Mat3,
    Mat4
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSGMeshVertexInput {
    // [index,] type, view_index, offset
    Position(RSGMeshVertexInputType, u32, usize),
    Normal(RSGMeshVertexInputType, u32, usize),
    Tangent(RSGMeshVertexInputType, u32, usize),
    Color(u32, RSGMeshVertexInputType, u32, usize),
    TexCoord(u32, RSGMeshVertexInputType, u32, usize),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RSGMeshBufferView {
    pub buffer_id: u32,
    pub offset: usize,
    pub size: usize,
    pub stride: usize
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSGMeshIndexBufferView {
    U16(RSGMeshBufferView),
    U32(RSGMeshBufferView)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSGMeshTopology {
    Triangles,
    TriangleStrip,
    Lines,
    LineStrip,
    Points
}

#[derive(Clone, Debug, PartialEq)]
pub struct RSGSubMesh {
    pub topology: RSGMeshTopology,
    pub vertex_count: u32,
    pub inputs: smallvec::SmallVec<[RSGMeshVertexInput; 8]>,
    pub index_count: Option<u32>,
    pub index_view: Option<RSGMeshIndexBufferView>
}

#[derive(Clone, Debug, PartialEq)]
pub struct RSGMesh {
    pub vertex_views: smallvec::SmallVec<[RSGMeshBufferView; 8]>,
    pub submeshes: smallvec::SmallVec<[RSGSubMesh; 1]>,
    pub bounds: RSGAabb
}

pub type RSGMeshComponentData = slotmap::SecondaryMap<RSGMeshKey, RSGMesh>;

#[derive(Clone, Debug, PartialEq)]
pub struct RSGMeshBuffer {
    pub data: Vec<f32>,
    pub source: String
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RSGAabb {
    pub minimum: glm::Vec3,
    pub maximum: glm::Vec3
}

impl RSGAabb {
    pub fn center(&self) -> glm::Vec3 {
        (self.minimum + self.maximum) * 0.5
    }
}

impl Default for RSGAabb {
    fn default() -> Self {
        RSGAabb { minimum: glm::zero(), maximum: glm::zero() }
    }
}

impl std::fmt::Display for RSGAabb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(min=[{}, {}, {}], max=[{}, {}, {}])",
            self.minimum.x, self.minimum.y, self.minimum.z,
            self.maximum.x, self.maximum.y, self.maximum.z)
    }
}

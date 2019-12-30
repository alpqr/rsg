use nalgebra_glm as glm;

slotmap::new_key_type! {
    pub struct RSGMaterialKey;
}

#[derive(Clone, Copy)]
pub struct RSGMaterialComponent {
}

impl RSGMaterialComponent {
    pub fn new() -> Self {
        RSGMaterialComponent {
        }
    }
}

pub type RSGMaterialComponentList = slotmap::SlotMap<RSGMaterialKey, RSGMaterialComponent>;

#[derive(Clone, Debug, PartialEq)]
pub enum RSGMaterialProperty {
    // name, default_value
    Float(String, f32),
    Vec2(String, glm::Vec2),
    Vec3(String, glm::Vec3),
    Vec4(String, glm::Vec4),
    Int(String, i32),
    Int2(String, glm::IVec2),
    Int3(String, glm::IVec3),
    Int4(String, glm::IVec4),
    Mat2(String, glm::Mat2),
    Mat3(String, glm::Mat3),
    Mat4(String, glm::Mat4)
}

#[derive(Clone, Debug, PartialEq)]
pub struct RSGMaterialShaderSet {
    pub vertex_shader: String,
    pub fragment_shader: String,
    pub properties: Vec<RSGMaterialProperty>
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSGMaterialCustomValue {
    Float(f32),
    Vec2(glm::Vec2),
    Vec3(glm::Vec3),
    Vec4(glm::Vec4),
    Int(i32),
    Int2(glm::IVec2),
    Int3(glm::IVec3),
    Int4(glm::IVec4),
    Mat2(glm::Mat2),
    Mat3(glm::Mat3),
    Mat4(glm::Mat4)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSGMaterialBuiltinValue {
    ModelMatrix,
    ViewMatrix,
    ProjectionMatrix,
    ModelViewMatrix,
    ViewProjectionMatrix,
    ModelViewProjectionMatrix,
    NormalMatrix
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSGMaterialPropertyValue {
    Builtin(RSGMaterialBuiltinValue),
    Custom(RSGMaterialCustomValue)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSGMaterialCullMode {
    None,
    Front,
    Back
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSGMaterialFrontFace {
    CCW,
    CW
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSGMaterialCompareOp {
    Never,
    Less,
    Equal,
    LessOrEqual,
    Greater,
    NotEqual,
    GreaterOrEqual,
    Always
}

bitflags::bitflags! {
    pub struct RSGMaterialColorMask: u32 {
        const R = 0x01;
        const G = 0x02;
        const B = 0x04;
        const A = 0x08;
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSGMaterialBlendFactor {
    Zero,
    One,
    SrcColor,
    OneMinusSrcColor,
    DstColor,
    OneMinusDstColor,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstAlpha,
    OneMinusDstAlpha,
    ConstantColor,
    OneMinusConstantColor,
    ConstantAlpha,
    OneMinusConstantAlpha,
    SrcAlphaSaturate,
    Src1Color,
    OneMinusSrc1Color,
    Src1Alpha,
    OneMinusSrc1Alpha
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSGMaterialBlendOp {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RSGMaterialBlend {
    pub color_write: RSGMaterialColorMask,
    pub blend_enable: bool,
    pub src_color: RSGMaterialBlendFactor,
    pub dst_color: RSGMaterialBlendFactor,
    pub op_color: RSGMaterialBlendOp,
    pub src_alpha: RSGMaterialBlendFactor,
    pub dst_alpha: RSGMaterialBlendFactor,
    pub op_alpha: RSGMaterialBlendOp
}

impl Default for RSGMaterialBlend {
    fn default() -> Self {
        RSGMaterialBlend {
            color_write: RSGMaterialColorMask::all(),
            blend_enable: false,
            src_color: RSGMaterialBlendFactor::One,
            dst_color: RSGMaterialBlendFactor::OneMinusSrcAlpha,
            op_color: RSGMaterialBlendOp::Add,
            src_alpha: RSGMaterialBlendFactor::One,
            dst_alpha: RSGMaterialBlendFactor::OneMinusSrcAlpha,
            op_alpha: RSGMaterialBlendOp::Add
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RSGMaterialGraphicsState {
    pub depth_test: bool,
    pub depth_write: bool,
    pub depth_op: RSGMaterialCompareOp,
    pub cull_mode: RSGMaterialCullMode,
    pub front_face: RSGMaterialFrontFace,
    pub blend: RSGMaterialBlend
}

impl Default for RSGMaterialGraphicsState {
    fn default() -> Self {
        RSGMaterialGraphicsState {
            depth_test: true,
            depth_write: true,
            depth_op: RSGMaterialCompareOp::Less,
            cull_mode: RSGMaterialCullMode::Back,
            front_face: RSGMaterialFrontFace::CCW,
            blend: Default::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RSGMaterial {
    pub shader_set_id: u32,
    pub property_values: std::collections::HashMap<String, RSGMaterialPropertyValue>,
    pub graphics_state: RSGMaterialGraphicsState
}

impl RSGMaterial {
    pub fn effective_graphics_state(&self, inherited_opacity: f32) -> RSGMaterialGraphicsState {
        let mut state = self.graphics_state;
        let has_transparency = inherited_opacity < 1.0 || state.blend.blend_enable;
        if has_transparency {
            state.depth_write = false; // no depth write in transparent pass
            if !state.blend.blend_enable { // force premul alpha when semi-transparent
                state.blend = Default::default();
                state.blend.blend_enable = true;
            }
        }
        state
    }
}

pub type RSGMaterialComponentData = slotmap::SecondaryMap<RSGMaterialKey, RSGMaterial>;

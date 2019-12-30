#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSGSamplerFilter {
    Nearest,
    Linear
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSGSamplerMipmapFilter {
    None,
    Nearest,
    Linear
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSGSamplerWrapMode {
    ClampToEdge,
    Repeat,
    MirroredRepeat
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RSGSampler {
    mag_filter: RSGSamplerFilter,
    min_filter: RSGSamplerFilter,
    min_mipmap_filter: RSGSamplerMipmapFilter,
    wrap_mode: RSGSamplerWrapMode
}

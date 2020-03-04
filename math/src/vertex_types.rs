use crate::{colors::RGBAColorF32, vec2::Vec2F32};

#[derive(Copy, Debug, Clone)]
pub struct VertexPC {
    pub pos: Vec2F32,
    pub color: RGBAColorF32,
}

impl std::default::Default for VertexPC {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::<VertexPC>::uninit().assume_init() }
    }
}

#[derive(Copy, Debug, Clone)]
pub struct VertexPTC {
    pub pos: Vec2F32,
    pub texcoords: Vec2F32,
    pub color: RGBAColorF32,
}

impl std::default::Default for VertexPTC {
    fn default() -> Self {
        unsafe { std::mem::MaybeUninit::<Self>::uninit().assume_init() }
    }
}

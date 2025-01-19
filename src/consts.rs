use crate::prelude::*;

pub const SCREEN_UVEC: UVec2 = UVec2::new(240, 240);
pub const SCREEN_VEC: Vec2 = Vec2::new(SCREEN_UVEC.x as f32, SCREEN_UVEC.y as f32);
pub const OVERLAY_GROWTH: u32 = 6;
pub const OVERLAY_UVEC: UVec2 = UVec2::new(
    SCREEN_UVEC.x * OVERLAY_GROWTH,
    SCREEN_UVEC.y * OVERLAY_GROWTH,
);
pub const OVERLAY_VEC: Vec2 = Vec2::new(OVERLAY_UVEC.x as f32, OVERLAY_UVEC.y as f32);

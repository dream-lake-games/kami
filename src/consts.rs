use egui::Color32;

use crate::prelude::*;

pub const SCREEN_UVEC: UVec2 = UVec2::new(240, 240);
pub const SCREEN_VEC: Vec2 = Vec2::new(SCREEN_UVEC.x as f32, SCREEN_UVEC.y as f32);
pub const OVERLAY_GROWTH: u32 = 6;
pub const OVERLAY_UVEC: UVec2 = UVec2::new(
    SCREEN_UVEC.x * OVERLAY_GROWTH,
    SCREEN_UVEC.y * OVERLAY_GROWTH,
);
pub const OVERLAY_VEC: Vec2 = Vec2::new(OVERLAY_UVEC.x as f32, OVERLAY_UVEC.y as f32);

// Convenience
pub const PI: f32 = std::f32::consts::PI;
#[expect(dead_code)]
pub const TAU: f32 = std::f32::consts::TAU;

/// Gameplay zixes
pub const ZIX_CHEF: f32 = 50.0;
pub const ZIX_ITEMS: f32 = 40.0;

/// Marked hboxes (hacking and totally arbitrary bc I forgot which (if any) numbers have special meaning whoops)
pub const HBOX_CAKE_GREEN: HBoxMarker = 10;
pub const HBOX_CAKE_BLUE: HBoxMarker = 11;
pub const HBOX_CAKE_PINK: HBoxMarker = 12;
pub const HBOX_CAKE_RED: HBoxMarker = 13;
pub const HBOX_DIRT_ROUGH: HBoxMarker = 21;
pub const HBOX_DIRT_SMOOTH: HBoxMarker = 22;

/// Root zixes
pub const ZIX_MENU: i32 = 100;
pub const ZIX_TRANSITION: i32 = 200;

/// Egui colors
pub const EGC1: Color32 = Color32::from_rgb(205, 212, 165);
pub const EGC2: Color32 = Color32::from_rgb(207, 169, 138);
pub const EGC3: Color32 = Color32::from_rgb(199, 120, 111);
pub const EGC6: Color32 = Color32::from_rgb(59, 64, 94);
pub const EGC7: Color32 = Color32::from_rgb(46, 42, 79);
pub const EGC8: Color32 = Color32::from_rgb(33, 29, 56);

/// Regular colors
pub const RGC1: Color = Color::srgb(205.0 / 255.0, 212.0 / 255.0, 165.0 / 255.0);
pub const RGC2: Color = Color::srgb(207.0 / 255.0, 169.0 / 255.0, 138.0 / 255.0);
pub const RGC3: Color = Color::srgb(199.0 / 255.0, 120.0 / 255.0, 111.0 / 255.0);
pub const RGC4: Color = Color::srgb(154.0 / 255.0, 98.0 / 255.0, 120.0 / 255.0);
pub const RGC5: Color = Color::srgb(96.0 / 255.0, 85.0 / 255.0, 110.0 / 255.0);
pub const RGC6: Color = Color::srgb(59.0 / 255.0, 64.0 / 255.0, 94.0 / 255.0);
pub const RGC7: Color = Color::srgb(46.0 / 255.0, 42.0 / 255.0, 79.0 / 255.0);
pub const RGC8: Color = Color::srgb(33.0 / 255.0, 29.0 / 255.0, 56.0 / 255.0);

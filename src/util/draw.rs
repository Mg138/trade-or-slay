use macroquad::prelude::{self, vec2, Color, DrawTextureParams, Texture2D, Vec2};
use macroquad::texture;

use crate::util::screen::world_size;

fn offsets() -> Vec<Vec2> {
    let (w, h) = world_size();

    vec![
        vec2(-w, -h),
        vec2(w, -h),
        vec2(-w, h),
        vec2(w, h),
        vec2(0.0, -h),
        vec2(0.0, h),
        vec2(-w, 0.0),
        vec2(w, 0.0),
        vec2(0.0, 0.0),
    ]
}

lazy_static! {
    static ref OFFSETS: Vec<Vec2> = offsets();
}

pub fn draw_texture_ex(
    texture: &Texture2D,
    x: f32,
    y: f32,
    color: Color,
    params: DrawTextureParams,
) -> Option<()> {
    OFFSETS.iter().for_each(|offset| {
        let (o_x, o_y) = (*offset).into();

        texture::draw_texture_ex(*texture, x + o_x, y + o_y, color, params.clone());
    });

    Some(())
}

pub fn draw_texture(texture: &Texture2D, x: f32, y: f32, color: Color) -> Option<()> {
    draw_texture_ex(texture, x, y, color, DrawTextureParams::default())
}

pub fn draw_line(x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) -> Option<()> {
    OFFSETS.iter().for_each(|offset| {
        let (o_x, o_y) = (*offset).into();

        prelude::draw_line(x1 + o_x, y1 + o_y, x2 + o_x, y2 + o_y, thickness, color);
    });

    Some(())
}

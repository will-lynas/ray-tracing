use std::{
    fmt::Debug,
    path::PathBuf,
};

use glam::{
    Vec2,
    Vec3A as Vec3,
};

use crate::color::Color;

pub trait Texture: Sync + Debug {
    fn value(&self, uv: Vec2, point: Vec3) -> Color;
}

#[derive(Debug, Clone, Copy)]
pub struct TextureHandle(usize);

#[derive(Default)]
pub struct TextureStore(Vec<Box<dyn Texture>>);

impl TextureStore {
    pub fn add(&mut self, texture: impl Texture + 'static) -> TextureHandle {
        let handle = self.0.len();
        self.0.push(Box::new(texture));
        TextureHandle(handle)
    }

    pub fn get(&self, handle: TextureHandle) -> &dyn Texture {
        self.0[handle.0].as_ref()
    }
}

#[derive(Debug)]
pub struct SolidColor {
    pub albedo: Color,
}

impl SolidColor {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self {
            albedo: Color::new(r, g, b),
        }
    }

    pub fn new_from_color(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn value(&self, _uv: Vec2, _point: Vec3) -> Color {
        self.albedo
    }
}

#[derive(Debug)]
pub struct CheckerTexture {
    odd: Box<dyn Texture>,
    even: Box<dyn Texture>,
    squares: f32,
}

impl CheckerTexture {
    pub fn new(odd: impl Texture + 'static, even: impl Texture + 'static, squares: f32) -> Self {
        Self {
            odd: Box::new(odd),
            even: Box::new(even),
            squares,
        }
    }

    pub fn new_from_color(odd: Color, even: Color, squares: f32) -> Self {
        Self::new(
            SolidColor::new_from_color(odd),
            SolidColor::new_from_color(even),
            squares,
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, uv: Vec2, point: Vec3) -> Color {
        let u = (uv.x * self.squares).floor() as i32;
        let v = (uv.y * self.squares).floor() as i32;

        if (u + v) % 2 == 0 {
            self.even.value(uv, point)
        } else {
            self.odd.value(uv, point)
        }
    }
}

#[derive(Debug)]
pub struct ImageTexture {
    image: image::RgbImage,
}

impl ImageTexture {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let image = image::open(path).unwrap();
        let rgb = image.to_rgb8();
        println!("width: {}, height: {}", rgb.width(), rgb.height());
        Self { image: rgb }
    }
}

impl Texture for ImageTexture {
    fn value(&self, uv: Vec2, _point: Vec3) -> Color {
        let u = (uv.x * self.image.width() as f32) as u32;
        let v = ((1.0 - uv.y) * self.image.height() as f32) as u32;
        self.image.get_pixel(u, v).into()
    }
}

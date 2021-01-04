use std::sync::Arc;

use crate::{hittable::T, perlin::*, vec3::*};

pub trait Texture {
    fn value(&self, tp: TexturePoint, p: Point3) -> Colour;
}

#[derive(Clone, Copy)]
pub struct TexturePoint {
    u: F,
    v: F,
}

impl TexturePoint {
    pub fn new(u: F, v: F) -> Self {
        TexturePoint { u, v }
    }

    pub fn u(&self) -> F { self.u }

    pub fn v(&self) -> F { self.v }
}

pub struct SolidColour {
    colour: Colour,
}

impl SolidColour {
    pub fn new(colour: Colour) -> Self {
        Self { colour }
    }

    pub fn rgb(r: F, g: F, b: F) -> Self {
        SolidColour::new(Colour::new(r, g, b))
    }
}

impl Texture for SolidColour {
    fn value(&self, _tp: TexturePoint, _p: Point3) -> Colour {
        self.colour
    }
}

pub struct Checkered {
    odd: Arc<T>,
    even: Arc<T>,
}

impl Checkered {
    pub fn new(odd: Arc<T>, even: Arc<T>) -> Self {
        Self { odd, even }
    }

    pub fn colour(odd_colour: Colour, even_colour: Colour) -> Self {
        let odd = Arc::new(SolidColour::new(odd_colour));
        let even = Arc::new(SolidColour::new(even_colour));

        Self { odd, even }
    }
}

impl Texture for Checkered {
    fn value(&self, tp: TexturePoint, p: Point3) -> Colour {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();

        if sines < 0.0 {
            self.odd.value(tp, p)
        } else {
            self.even.value(tp, p)
        }
    }
}

pub struct Noise {
    noise: Perlin,
    scale: F,
}

impl Noise {
    pub fn new(scale: F) -> Self {
        Self { noise: Perlin::new(), scale }
    }
}

impl Texture for Noise {
    fn value(&self, _tp: TexturePoint, p: Point3) -> Colour {
        Colour::new(0.18, 0.1, 0.28) * 0.5 * 
        (1.0 + (self.scale * p.z() + 10.0  * self.noise.turbulence(p * self.scale, 7)).sin())
    }
}
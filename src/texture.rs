use std::sync::Arc;

use crate::{hittable::T, vec3::*};

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
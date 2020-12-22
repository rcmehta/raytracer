use std::{fmt::Display, fmt::Formatter, fmt::Result, iter::Sum};
use std::ops::{Add, Sub, Mul, Div, Neg};

use rand::{Rng, thread_rng};
use rand_distr::StandardNormal;

pub type F = f64;
pub type Colour = Vec3;
pub type Point3 = Vec3;

#[derive(Copy, Clone)]
pub struct Vec3(F, F, F);

impl Vec3 {
    pub fn new(x: F, y: F, z: F) -> Vec3 {
        Vec3(x, y, z)
    }

    pub fn random_vector(min: F, max: F) -> Vec3 {
        Vec3(random_range(min, max), random_range(min, max), random_range(min, max))
    }

    pub fn random_on_unit_sphere() -> Vec3 {
        loop {
            let vec = Vec3::new(random_gaussian(), random_gaussian(), random_gaussian());
            if vec.x() != 0.0 && vec.y() != 0.0 && vec.z() != 0.0 {
                return vec / vec.length();
            }
        }
    }
    
    pub fn random_in_unit_disc() -> Vec3 {
        loop {
            let vec = Vec3::new(random_gaussian(), random_gaussian(), 0.0);
            if vec.x() != 0.0 && vec.y() != 0.0 {
                return vec / vec.length();
            }
        }
    }
    
    pub fn x(&self) -> F { self.0 }

    pub fn y(&self) -> F { self.1 }

    pub fn z(&self) -> F { self.2 }

    pub fn near_zero(&self) -> bool {
        let epsilon = 1e-10;
        self.length_squared() < epsilon
    }

    pub fn length_squared(&self) -> F {
        dot(&self, &self)
    }

    pub fn length(&self) -> F {
        self.length_squared().sqrt()
    }

    pub fn unit(&self) -> Vec3 {
        Vec3(self.0, self.1, self.2) / self.length()
    }

    pub fn write_colour(&self, samples_per_pixel: u32) -> String {
        let scale = 1.0 / samples_per_pixel as F;
        // Divide colour by number of samples and gamma-correct for gamma = 2.0
        let colour = Vec3::new(
            (self.0 * scale).sqrt(),
            (self.1 * scale).sqrt(),
            (self.2 * scale).sqrt());
        format!("{} {} {}", 
        (clamp(colour.0, 0.0, 0.999) * 256.0).floor(),
        (clamp(colour.1, 0.0, 0.999) * 256.0).floor(),
        (clamp(colour.2, 0.0, 0.999) * 256.0).floor())
    }

}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self.0, self.1, self.2)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self {
        Vec3(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self {
        Vec3(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl Mul<F> for Vec3 {
    type Output = Self;

    fn mul(self, lambda: F) -> Self {
        Vec3(self.0 * lambda, self.1 * lambda, self.2 * lambda)
    }
}

impl Div<F> for Vec3 {
    type Output = Self;

    fn div(self, lambda: F) -> Self {
        if lambda == 0.0 {
            panic!("Cannot divide by zero.");
        }
        Vec3(self.0 / lambda, self.1 / lambda, self.2 / lambda)
    }
}

impl Sum<Vec3> for Vec3 {
    fn sum<I> (iter: I) -> Vec3 where I: Iterator<Item=Vec3> {
        iter.fold(Vec3(0.0, 0.0, 0.0), |u, v| Vec3(
            u.x() + v.x(), u.y() + v.y(), u.z() + v.z()
        ))
    }
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3(
        (u.1 * v.2) - (u.2 * v.1),
        (u.2 * v.0) - (u.0 * v.2),
        (u.0 * v.1) - (u.1 * v.0),
    )
}

pub fn dot(u: &Vec3, v: &Vec3) -> F {
    (u.0 * v.0) + (u.1 * v.1) + (u.2 * v.2)
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - n * dot(&v, &n) * 2.0
}

pub fn refract(ray_in: Vec3, n: Vec3, eta_over_eta_prime: F) -> Vec3 {
    let cos_theta = dot(&(-ray_in), &n).min(1.0);
    let r_out_perp = (ray_in + n * cos_theta) * eta_over_eta_prime;
    let r_out_parallel = n * -((1.0 - r_out_perp.length_squared()).abs().sqrt());
    r_out_perp + r_out_parallel
}

pub fn clamp(x: F, min: F, max: F) -> F {
    if x < min { return min; }
    else if x > max { return max; }
    else { return x; }
}

// Random
pub fn random() -> F {
    thread_rng().gen::<F>()
}

pub fn random_range(min: F, max: F) -> F {
    min + (max - min) * random()
}

pub fn random_gaussian() -> F {
    thread_rng().sample(StandardNormal)
}
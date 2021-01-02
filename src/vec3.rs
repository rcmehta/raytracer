use std::ops::{Add, Sub, Neg, Div, Mul};
use std::{fmt::Display, fmt::Formatter, fmt::Result, iter::Sum};

use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;

pub type F = f64;
pub type Colour = Vec3;
pub type Point3 = Vec3;

#[derive(Copy, Clone)]
pub struct Vec3([F; 3]);

impl Vec3 {
    pub fn new(x: F, y: F, z: F) -> Self {
        Vec3([x, y, z])
    }

    pub fn random_vector(min: F, max: F) -> Self {
        Vec3::new(
            random_range(min, max),
            random_range(min, max),
            random_range(min, max),
        )
    }

    pub fn random_on_unit_sphere() -> Self {
        loop {
            let vec = Vec3::new(random_gaussian(), random_gaussian(), random_gaussian());
            if vec.length_squared() > 0.0 {
                return vec.unit();
            }
        }
    }

    pub fn random_in_unit_disc() -> Self {
        loop {
            let vec = Vec3::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
            if vec.length_squared() < 1.0 {
                return vec;
            }
        }
    }

    pub fn x(&self) -> F {
        self.0[0]
    }

    pub fn y(&self) -> F {
        self.0[1]
    }

    pub fn z(&self) -> F {
        self.0[2]
    }

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

    pub fn unit(self) -> Self {
        self / self.length()
    }

    pub fn write_colour(&self, samples_per_pixel: u32) -> String {
        let scale = 1.0 / samples_per_pixel as F;
        // Divide colour by number of samples and gamma-correct for gamma = 2.0
        let colour = Vec3::new(
            (self.x() * scale).sqrt(),
            (self.y() * scale).sqrt(),
            (self.z() * scale).sqrt(),
        );
        format!(
            "{} {} {}",
            (clamp(colour.x(), 0.0, 0.999) * 256.0).floor(),
            (clamp(colour.y(), 0.0, 0.999) * 256.0).floor(),
            (clamp(colour.z(), 0.0, 0.999) * 256.0).floor()
        )
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl Into<[F; 3]> for Vec3 {
    fn into(self) -> [F; 3] {
        self.0
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Self {
        Vec3::new(self.x() + other.x(), self.x() + other.y(), self.z() + other.z())
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self {
        Vec3::new(self.x() - other.x(), self.y() - other.y(), self.z() - other.z())
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Vec3::new(self.x() * other.x(), self.y() * other.y(), self.z() * other.z())
    }
}

impl Mul<F> for Vec3 {
    type Output = Self;

    fn mul(self, lambda: F) -> Self {
        Vec3::new(self.x() * lambda, self.y() * lambda, self.z() * lambda)
    }
}

impl Div<F> for Vec3 {
    type Output = Self;

    fn div(self, lambda: F) -> Self {
        if lambda == 0.0 {
            panic!("Cannot divide by zero.");
        }
        Vec3::new(self.x() / lambda, self.y() / lambda, self.z() / lambda)
    }
}

impl Sum<Vec3> for Vec3 {
    fn sum<I>(iter: I) -> Vec3
    where
        I: Iterator<Item = Vec3>,
    {
        iter.fold(Vec3::new(0.0, 0.0, 0.0), |u, v| {
            Vec3::new(u.x() + v.x(), u.y() + v.y(), u.z() + v.z())
        })
    }
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(
        (u.y() * v.z()) - (u.z() * v.y()),
        (u.z() * v.x()) - (u.x() * v.z()),
        (u.x() * v.y()) - (u.y() * v.x()),
    )
}

pub fn dot(u: &Vec3, v: &Vec3) -> F {
    (u.x() * v.x()) + (u.y() * v.y()) + (u.z() * v.z())
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
    if x < min {
        return min;
    } else if x > max {
        return max;
    } else {
        return x;
    }
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

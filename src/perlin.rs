use rand::{prelude::SliceRandom, thread_rng};

use crate::vec3::*;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    random_vectors: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let random_vectors: Vec<Vec3> = (0..POINT_COUNT).map(|_| Vec3::random_vector(-1.0, 1.0)).collect();
        let perm_x = Perlin::generate_perm();
        let perm_y = Perlin::generate_perm();
        let perm_z = Perlin::generate_perm();

        Self { random_vectors, perm_x, perm_y, perm_z }
    }

    fn generate_perm() -> Vec<usize> {
        let mut rng = thread_rng();
        let mut perm: Vec<usize> = (0..POINT_COUNT).map(|i| i).collect();
        perm.shuffle(&mut rng);

        perm
    }

    pub fn noise(&self, p: Point3) -> F {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::zero(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_vectors[(
                        self.perm_x[(i + di as i32) as usize & 255] ^
                        self.perm_y[(j + dj as i32) as usize & 255] ^
                        self.perm_z[(k + dk as i32) as usize & 255]
                    ) as usize
                    ];
                }
            }
        }

        Perlin::perlin_interpolation(c, u, v, w)
    }

    fn perlin_interpolation(c: [[[Vec3; 2]; 2]; 2], u: F, v: F, w: F) -> F {
        let mut accum = 0.0;

        // Hermite cubic
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as F, v - j as F, w - k as F);
                    accum +=
                    (i as F * uu + (1.0 - uu) * (1 - i) as F) *
                    (j as F * vv + (1.0 - vv) * (1 - j) as F) *
                    (k as F * ww + (1.0 - ww) * (1 - k) as F) * dot(&c[i][j][k], &weight_v);
                }
            }
        }

        accum
    }

    pub fn turbulence(&self, p: Point3, depth: usize) -> F {
        let mut accum = 0.0;
        let mut temp = p;
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(temp);
            weight *= 0.5;
            temp = temp * 2.0;
        }

        accum.abs()
    }
}
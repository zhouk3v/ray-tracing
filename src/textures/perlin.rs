use crate::primitives::point3::Point3;
use crate::primitives::vec3::{dot, unit_vector, Vec3};

pub struct Perlin {
    randvec: [Vec3; 256],
    perm_x: [i32; 256],
    perm_y: [i32; 256],
    perm_z: [i32; 256],
}

impl Perlin {
    pub fn new() -> Self {
        let mut randvec = [Vec3::default(); 256];

        for i in randvec.iter_mut() {
            *i = unit_vector(Vec3::random_with_min_max(-1.0, 1.0));
        }

        Self {
            randvec,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for (di, c1) in c.iter_mut().enumerate() {
            for (dj, c2) in c1.iter_mut().enumerate() {
                for (dk, c3) in c2.iter_mut().enumerate() {
                    let perm_x_index = (i + di as i32) & 255;
                    let perm_y_index = (j + dj as i32) & 255;
                    let perm_z_index = (k + dk as i32) & 255;
                    let index = self.perm_x[perm_x_index as usize]
                        ^ self.perm_y[perm_y_index as usize]
                        ^ self.perm_z[perm_z_index as usize];
                    *c3 = self.randvec[index as usize];
                }
            }
        }

        Self::perlin_interp(c, u, v, w)
    }

    fn perlin_generate_perm() -> [i32; 256] {
        let mut perm = [0; 256];
        for (i, p) in perm.iter_mut().enumerate() {
            *p = i as i32;
        }
        Self::permute(&mut perm, 256);
        perm
    }

    fn permute(p: &mut [i32; 256], n: usize) {
        for i in (0..n - 1).rev() {
            let target = rand::random_range(0..=i);
            p.swap(i, target);
        }
    }

    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for (i, c1) in c.iter().enumerate() {
            for (j, c2) in c1.iter().enumerate() {
                for (k, c3) in c2.iter().enumerate() {
                    let i_double = i as f64;
                    let j_double = j as f64;
                    let k_double = k as f64;
                    let weight_v = Vec3::new(u - i_double, v - j_double, w - k_double);
                    accum += (i_double * uu + (1.0 - i_double) * (1.0 - uu))
                        * (j_double * vv + (1.0 - j_double) * (1.0 - vv))
                        * (k_double * ww + (1.0 - k_double) * (1.0 - ww))
                        * dot(c3, &weight_v)
                }
            }
        }
        accum
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

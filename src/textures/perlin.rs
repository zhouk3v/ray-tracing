use crate::primitives::point3::Point3;

pub struct Perlin {
    randfloat: [f64; 256],
    perm_x: [i32; 256],
    perm_y: [i32; 256],
    perm_z: [i32; 256],
}

impl Perlin {
    pub fn new() -> Self {
        let mut randfloat = [0.0; 256];
        for i in randfloat.iter_mut() {
            *i = rand::random::<f64>();
        }

        Self {
            randfloat,
            perm_x: Self::perlin_generate_perm(),
            perm_y: Self::perlin_generate_perm(),
            perm_z: Self::perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        // let i = (4.0 * p.x()) as i32 & 255;
        // let j = (4.0 * p.y()) as i32 & 255;
        // let k = (4.0 * p.z()) as i32 & 255;

        // let index = self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize];

        // self.randfloat[index as usize]
        let u_base = p.x() - p.x().floor();
        let v_base = p.y() - p.y().floor();
        let w_base = p.z() - p.z().floor();

        let u = u_base * u_base * (3.0 - 2.0 * u_base);
        let v = v_base * v_base * (3.0 - 2.0 * v_base);
        let w = w_base * w_base * (3.0 - 2.0 * w_base);

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c = [[[0.0; 2]; 2]; 2];

        for (di, c1) in c.iter_mut().enumerate() {
            for (dj, c2) in c1.iter_mut().enumerate() {
                for (dk, c3) in c2.iter_mut().enumerate() {
                    let perm_x_index = (i + di as i32) & 255;
                    let perm_y_index = (j + dj as i32) & 255;
                    let perm_z_index = (k + dk as i32) & 255;
                    let index = self.perm_x[perm_x_index as usize]
                        ^ self.perm_y[perm_y_index as usize]
                        ^ self.perm_z[perm_z_index as usize];
                    *c3 = self.randfloat[index as usize];
                }
            }
        }

        Self::trilinear_interp(c, u, v, w)
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

    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;
        for (i, c1) in c.iter().enumerate() {
            for (j, c2) in c1.iter().enumerate() {
                for (k, c3) in c2.iter().enumerate() {
                    let i_double = i as f64;
                    let j_double = j as f64;
                    let k_double = k as f64;
                    accum += (i_double * u + (1.0 - i_double) * (1.0 - u))
                        * (j_double * v + (1.0 - j_double) * (1.0 - v))
                        * (k_double * w + (1.0 - k_double) * (1.0 - w))
                        * c3
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

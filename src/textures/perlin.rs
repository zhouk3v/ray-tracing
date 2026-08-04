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
        let i = (4.0 * p.x()) as i32 & 255;
        let j = (4.0 * p.y()) as i32 & 255;
        let k = (4.0 * p.z()) as i32 & 255;

        let index = self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize];

        self.randfloat[index as usize]
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
}

impl Default for Perlin {
    fn default() -> Self {
        Self::new()
    }
}

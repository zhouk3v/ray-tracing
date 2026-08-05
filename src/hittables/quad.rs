use super::hittable::{HitRecord, Hittable};
use crate::materials::material::Material;
use crate::primitives::aabb::Aabb;
use crate::primitives::interval::Interval;
use crate::primitives::point3::Point3;
use crate::primitives::ray::Ray;
use crate::primitives::vec3::{cross, dot, unit_vector, Vec3};

struct QuadUV {
    u: f64,
    v: f64,
}

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Box<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Box<dyn Material>) -> Self {
        let bbox_diagonal1 = Aabb::new_from_points(&q, &(q + u + v));
        let bbox_diagonal2 = Aabb::new_from_points(&(q + u), &(q + v));
        let n = cross(&u, &v);
        let normal = unit_vector(n);
        Self {
            q,
            u,
            v,
            w: n / dot(&n, &n),
            mat,
            bbox: Aabb::new_from_aabb(&bbox_diagonal1, &bbox_diagonal2),
            normal,
            d: dot(&normal, &q),
        }
    }

    fn is_interior(&self, a: f64, b: f64) -> Option<QuadUV> {
        let unit_interval = Interval::new(0.0, 1.0);
        // Given the hit point in plane coordinates, return None if it is outside the
        // primitive, otherwise return the hit record UV coordinates.

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            None
        } else {
            Some(QuadUV { u: a, v: b })
        }
    }
}

impl Hittable for Quad {
    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
        let denom = dot(&self.normal, r.direction());

        // No hit if the ray is parallel to the plane.
        if denom.abs() < 1.0e-8 {
            None
        } else {
            let t = (self.d - dot(&self.normal, r.origin())) / denom;
            // No hit if the hit point parameter t is outside the ray interval.
            if !ray_t.contains(t) {
                None
            } else {
                // Determine if the hit point lies within the planar shape using its plane coordinates.
                let intersection = r.at(t);
                let planar_hitpt_vector = intersection - self.q;
                let alpha = dot(&self.w, &cross(&planar_hitpt_vector, &self.v));
                let beta = dot(&self.w, &cross(&self.u, &planar_hitpt_vector));

                if let Some(uv) = self.is_interior(alpha, beta) {
                    // Ray hits the 2D shape; set the rest of the hit record and return true.
                    let rec = HitRecord::new(t, r, self.normal, self.mat.as_ref(), uv.u, uv.v);
                    Some(rec)
                } else {
                    None
                }
            }
        }
    }
}

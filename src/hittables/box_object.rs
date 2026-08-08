use crate::hittables::hittable::{HitRecord, Hittable};
use crate::hittables::hittable_list::HittableList;
use crate::hittables::quad::Quad;
use crate::materials::material::Material;
use crate::primitives::aabb::Aabb;
use crate::primitives::interval::Interval;
use crate::primitives::point3::Point3;
use crate::primitives::ray::Ray;
use crate::primitives::vec3::Vec3;

pub struct BoxInstanceSideMaterials {
    front: Box<dyn Material>,
    right: Box<dyn Material>,
    back: Box<dyn Material>,
    left: Box<dyn Material>,
    top: Box<dyn Material>,
    bottom: Box<dyn Material>,
}

impl BoxInstanceSideMaterials {
    pub fn new(
        front: Box<dyn Material>,
        right: Box<dyn Material>,
        back: Box<dyn Material>,
        left: Box<dyn Material>,
        top: Box<dyn Material>,
        bottom: Box<dyn Material>,
    ) -> Self {
        Self {
            front,
            right,
            back,
            left,
            top,
            bottom,
        }
    }
}

pub struct BoxInstance {
    sides: HittableList,
}

impl BoxInstance {
    pub fn new(a: &Point3, b: &Point3, mat: BoxInstanceSideMaterials) -> Self {
        let mut sides = HittableList::new();

        // Construct the two opposite vertices with the minimum and maximum coordinates
        let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
        let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

        let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
        let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
        let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

        sides.add(Box::new(Quad::new(
            Point3::new(min.x(), min.y(), max.z()),
            dx,
            dy,
            mat.front,
        )));
        sides.add(Box::new(Quad::new(
            Point3::new(max.x(), min.y(), max.z()),
            -dz,
            dy,
            mat.right,
        )));
        sides.add(Box::new(Quad::new(
            Point3::new(max.x(), min.y(), min.z()),
            -dx,
            dy,
            mat.back,
        )));
        sides.add(Box::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dz,
            dy,
            mat.left,
        )));
        sides.add(Box::new(Quad::new(
            Point3::new(min.x(), max.y(), max.z()),
            dx,
            -dz,
            mat.top,
        )));
        sides.add(Box::new(Quad::new(
            Point3::new(min.x(), min.y(), min.z()),
            dx,
            dz,
            mat.bottom,
        )));

        Self { sides }
    }
}

impl Hittable for BoxInstance {
    fn bounding_box(&self) -> &Aabb {
        self.sides.bounding_box()
    }

    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord<'_>> {
        self.sides.hit(r, ray_t)
    }
}

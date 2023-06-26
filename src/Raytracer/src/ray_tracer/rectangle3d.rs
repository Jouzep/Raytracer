//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Rectangle
//

use crate::math::point3d::Point3D;
use crate::math::vector3d::Vector3D;

#[derive(Copy, Clone)]
pub struct Rectangle3D {
    origin:Point3D,
    bottom_side:Vector3D,
    left_side:Vector3D
}

impl Rectangle3D {
    pub fn new(origin:Point3D, bottom_side:Vector3D, left_side:Vector3D) -> Self {
        Rectangle3D { origin, bottom_side, left_side }
    }
    pub fn point_at(&self, u:f64, v:f64) -> Point3D {
        let x = self.origin.x + u * self.bottom_side.x + v * self.left_side.x;
        let y = self.origin.y + u * self.bottom_side.y + v * self.left_side.y;
        let z = self.origin.z + u * self.bottom_side.z + v * self.left_side.z;
        return Point3D::new(x, y, z);
    }
}

impl Default for Rectangle3D {
    fn default() -> Self {
        Rectangle3D {
            origin: Point3D::default(),
            bottom_side: Vector3D::default(),
            left_side: Vector3D::default()

        }
    }
}
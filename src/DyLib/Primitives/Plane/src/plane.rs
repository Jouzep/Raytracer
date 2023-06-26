//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// planes
//

use crate::math::{vector3d::Vector3D, point3d::Point3D};
use crate::interfaces::primitives::Primitives;
use crate::ray_tracer::ray::Ray;

#[derive(Copy, Clone)]
pub struct Plane {
    pub axis:char,
    pub center:Point3D,
    pub direction:Vector3D,
    pub color:Vector3D
}

impl Plane {
    pub fn new_config(axis:char, position:f64, color:Vector3D) -> Self {
        let mut pos = Point3D::default();
        let mut direction = Vector3D::default();
        if axis == 'X' {
            pos.x = position;
            direction.x = 1.0;
        }
        if axis == 'Y' {
            pos.y = position;
            direction.y = 1.0;
        }
        if axis == 'Z' {
            pos.z = position;
            direction.y = 1.0;
        }
        Plane {axis, center:pos, direction, color}
    }
}

impl Primitives for Plane {
    fn hits(&self, ray:Ray) -> Option<Point3D>{
        let dot = ray.direction.scal(&self.direction);

        if dot > 0.0 {
            let t = ((self.center - ray.origin).scal(&self.direction)) / dot;
            if t > 0.0 {
                return None;
            }
        }
        return None;
    }
    fn translate(&mut self, translate:Vector3D) {
        self.center.x += translate.x;
        self.center.y += translate.y;
        self.center.z += translate.z;
    }
    fn rotatex(&mut self, _:f64) {}
    fn rotatey(&mut self, _:f64) {}
    fn rotatez(&mut self, _:f64) {}
    fn suface_normal(&self, _:Point3D) -> Vector3D {
        Vector3D::new(0.0, 1.0, 0.1)
    }
    fn get_color(&self) -> Vector3D {
        self.color
    }
}

impl Default for Plane {
    fn default() -> Self {
        Plane { axis: 'X', center: Point3D::default(), direction: Vector3D::default(), color: Vector3D::default() }
    }
}


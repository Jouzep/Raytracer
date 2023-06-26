//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Camera
//

use crate::math::point3d::Point3D;
use crate::math::vector3d::Vector3D;
use crate::ray_tracer::rectangle3d::Rectangle3D;
use crate::ray_tracer::ray::Ray;

#[derive(Copy, Clone)]
pub struct Camera {
    origin:Point3D,
    screen:Rectangle3D,
    pub width:u32,
    pub height:u32,
    pub rotation:Vector3D,
    pub fov:f64
}

impl Camera {
    pub fn new_config(width:u32, height:u32, position:Point3D, rotation:Vector3D, fov:f64) -> Self{
        let dis = (width / 2) as f64 / (fov / 2.0).to_radians().tan();
        let bottom_side = Vector3D::new(width as f64, 0.0, 0.0);
        let left_side = Vector3D::new(0.0, height as f64, 0.0);
        let screen_origin = Point3D::new(
            -(((width) as f64) / 2.0 - position.x),
            -((height as f64) / 2.0 - position.y),
            position.z);
        Camera {
            origin: Point3D::new(position.x, position.y, position.z - dis),
            screen: Rectangle3D::new(screen_origin, bottom_side, left_side),
            width,
            height,
            rotation,
            fov
        }
    }

    pub fn ray(&self, u:f64, v:f64) -> Ray {
        let point = self.screen.point_at(u, v);
        Ray { origin:self.origin, direction: (point - self.origin) }
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            origin:Point3D::default(),
            screen:Rectangle3D::default(),
            width:0,
            height:0,
            rotation:Vector3D::default(),
            fov:0.0
        }
    }
}

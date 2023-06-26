//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// sphere
//

use crate::math::{point3d::Point3D, formulas, vector3d::Vector3D};
use crate::ray_tracer::ray::Ray;
use crate::interfaces::{Primitives, Mask};
use crate::canvas::material::Solid;
use crate::matrix::Transformation;

use super::color::Color;

#[derive(Clone)]
pub struct Sphere {
    center:Point3D,
    radius:f64,
    color:Color,
    pattern:Box<dyn Mask>,
    reflectiveness: f64,
    transformation: Transformation
}

// impl Sphere {
//     pub fn new_config(center:Point3D, radius:f64, color:Color, pattern:Box<dyn Mask>, reflectiveness:f64) -> Self {
//         Sphere {center, radius, color, pattern, reflectiveness}
//     }
// }


impl Primitives for Sphere {
    fn hits(&self, ray:Ray) -> Option<Point3D> {
        let dif = ray.origin - self.center;
        let a = ray.direction.scal(&ray.direction);
        let b = 2.0 * ray.direction.scal(&dif);
        let c = dif.scal(&dif) - self.radius.powf(2.0);
        let des = formulas::compute_discriminant(a, b, c);
        let res = formulas::resolve_quadratic_eq(des, a, b);
        if let Some(v) = res {
            let inter_points = formulas::get_inter_point_from_eq(v, ray.origin, ray.direction);
            return Some(formulas::get_closest_point(inter_points, ray.origin));
        }
        None
    }
    fn translate(&mut self, translate:Vector3D) {
        self.center.x += &translate.x;
        self.center.y += &translate.y;
        self.center.z += &translate.z;
    }
    fn rotatex(&mut self, _:f64) {}
    fn rotatey(&mut self, _:f64) {}
    fn rotatez(&mut self, _:f64) {}
    fn scale(&mut self, value:f64) {
        self.radius *= value;
    }
    fn suface_normal(&self, hit_point:Point3D) -> Vector3D {
        (hit_point - self.center).normalize()
    }
    fn get_color(&self) -> Color {
        self.color
    }

    fn get_pattern(&self) -> Box<dyn Mask> {
        self.pattern.clone()
    }
    fn get_reflectiveness(&self) -> f64 {
        self.reflectiveness
    }

    fn clone_box(&self) -> Box<dyn Primitives> {
        Box::new(Self {
            center: self.center,
            color: self.color,
            radius: self.radius,
            pattern: self.pattern.clone(),
            reflectiveness: self.reflectiveness,
            transformation: self.transformation
        })
    }

    fn with_center(&mut self, center:Option<Point3D>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if center.is_none() {
            return Err("Missing center".into());
        }
        self.center = center.unwrap();
        Ok(())
    }

    fn with_radius(&mut self, radius:Option<f64>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if radius.is_none() {
            return Err("Missing center".into());
        }
        self.radius = radius.unwrap();
        Ok(())
    }

    fn with_color(&mut self, color:Option<Color>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if color.is_none() {
            return Err("Missing color".into());
        }
        self.color = color.unwrap();
        Ok(())
    }

    fn with_pattern(&mut self, pattern:Option<Box<dyn Mask>>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if pattern.is_none() {
            self.pattern = Box::new(Solid::new(self.color));
        } else {
            self.pattern = pattern.unwrap();
        }
        Ok(())
    }

    fn with_reflectiveness(&mut self, reflectiveness:Option<f64>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if reflectiveness.is_none() {
            self.reflectiveness = 0.0;
        } else {
            self.reflectiveness = reflectiveness.unwrap();
        }
        Ok(())
    }

    fn with_axis(&mut self, _axis:Option<char>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn with_height(&mut self, _height:Option<f64>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    fn with_scale(&mut self, scale:Option<f64>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if scale.is_none() {
            self.transformation.scale = 1.0;
        } else {
            self.transformation.scale = scale.unwrap();
        }
        Ok(())
    }

    fn with_translation(&mut self, translation:Option<Vector3D>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if translation.is_none() {
            self.transformation.translation = Vector3D::default();
        } else {
            self.transformation.translation = translation.unwrap();
        }
        Ok(())
    }

    fn with_rotation(&mut self, rotation:Option<Vector3D>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if rotation.is_none() {
            self.transformation.rotation = Vector3D::default();
        } else {
            self.transformation.rotation = rotation.unwrap();
        }
        Ok(())
    }

}

// impl Clone for Sphere {
//     fn clone(&self) -> Self {
//         Sphere {
//             center: self.center.clone(),
//             radius: self.radius,
//             color: self.color.clone(),
//             pattern: self.pattern.clone()
//         }
//     }
// }

impl Default for Sphere {
    fn default() -> Self {
        Sphere {
            center: Point3D::default(),
            radius: 0.0,
            color: Color::default(),
            pattern: Box::new(Solid::default()),
            reflectiveness: 0.0,
            transformation: Transformation::default()
        }
    }
}
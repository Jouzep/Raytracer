//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Cylinder
//

use crate::math::{vector3d::Vector3D, point3d::Point3D, formulas};
use crate::interfaces::{Primitives, Mask};
use crate::matrix::Transformation;
use crate::ray_tracer::ray::Ray;
use super::color::Color;
use super::material::{
    Solid,
};

#[derive(Clone)]
pub struct Cylinder {
    pub center:Point3D,
    pub radius:f64,
    pub height:f64,
    pub color:Color,
    pub axis:char,
    pub pattern:Box<dyn Mask>,
    pub reflectiveness:f64,
    transformation:Transformation
}

impl Cylinder {
    // pub fn new_config(center:Point3D, radius:f64, height: f64, color:Color, axis:char, pattern:Box<dyn Mask>) -> Self {
    //     Cylinder {center, radius, height, color, axis, pattern, reflectiveness: 0.0}
    // }

    fn get_origin(&self, ray:Ray) -> Option<Vec<f64>> {
        if self.axis == 'X' {
            return Some(vec![ray.origin.y, ray.origin.z, ray.origin.x]);
        } else if self.axis == 'Y' {
            return Some(vec![ray.origin.x, ray.origin.z, ray.origin.y]);
        } else if self.axis == 'Z' {
            return Some(vec![ray.origin.x, ray.origin.y, ray.origin.z]);
        } else {
            return None;
        }
    }

    fn get_center(&self) -> Option<Vec<f64>> {
        if self.axis == 'X' {
            return Some(vec![self.center.y, self.center.z, self.center.x]);
        } else if self.axis == 'Y' {
            return Some(vec![self.center.x, self.center.z, self.center.y]);
        } else if self.axis == 'Z' {
            return Some(vec![self.center.x, self.center.y, self.center.z]);
        } else {
            return None;
        }
    }

    fn get_direction(&self, ray:Ray) -> Option<Vec<f64>> {
        if self.axis == 'X' {
            return Some(vec![ray.direction.y, ray.direction.z, ray.direction.x]);
        } else if self.axis == 'Y' {
            return Some(vec![ray.direction.x, ray.direction.z, ray.direction.y]);
        } else if self.axis == 'Z' {
            return Some(vec![ray.direction.x, ray.direction.y, ray.direction.z]);
        } else {
            return None;
        }
    }

    fn compute_cylinder_surface(&self, origin: &Vec<f64>, center: &Vec<f64>, direction: &Vec<f64>) -> Option<f64> {
        let a: f64 = direction[0].powi(2) + direction[1].powi(2);

        let b: f64 = 2.0 * (direction[0] * (origin[0] - center[0])
        + direction[1] * (origin[1] - center[1]));

        let c: f64 = (origin[0] - center[0]).powi(2) +
        (origin[1] - center[1]).powi(2) - self.radius.powi(2);

        let delta = formulas::compute_discriminant(a, b, c);
        if delta.abs() < 0.001 || delta < 0.0 {
            return None;
        }
        let t1: f64 = (-b - delta.sqrt()) / (2.0 * a);
        let t2: f64 = (-b + delta.sqrt()) / (2.0 * a);
        let t: f64 = t1.min(t2);

        let r = origin[2] + t * direction[2];
        if r >= center[2] && r <= center[2] + self.height {
            return Some(r);
        }
        None
    }

    fn compute_last_cylinder_surface(&self, origin: &Vec<f64>, center: &Vec<f64>, direction: &Vec<f64>) -> Option<f64> {
        let t1: f64 = (center[2] - origin[2]) / direction[2];
        let t2: f64 = (center[2] + self.height - origin[2]) / direction[2];
        let mut t: f64 = -1.0;

        if t1 > 0.0 {
            let p1 = origin[0] + t1 * direction[0] - center[0];
            let p2 = origin[1] + t1 * direction[1] - center[1];
            if p1.powi(2) + p2.powi(2) <= self.radius.powi(2) {
                t = t1;
            }
        }
        if t2 > 0.0 {
            let p1 = origin[0] + t2 * direction[0] - center[0];
            let p2 = origin[1] + t2 * direction[1] - center[1];
            if p1.powi(2) + p2.powi(2) <= self.radius.powi(2) &&
            (t < 0.0 || t2 < t)  {
                t = t2;
            }
        }
        if t > 0.0 {
            return Some(t);
        }
        None
    }
}

impl Primitives for Cylinder {
    fn hits(&self, ray: Ray) -> Option<Point3D> {
        let origin = match self.get_origin(ray) {
            Some(result) => result,
            None => return None,
        };
        let center = match self.get_center() {
            Some(result) => result,
            None => return None,
        };
        let direction = match self.get_direction(ray) {
            Some(result) => result,
            None => return None,
        };

        if let Some(t) = self.compute_cylinder_surface(&origin, &center, &direction) {
            return Some(Point3D::new(
                ray.origin.x + t * ray.direction.x,
                ray.origin.y + t * ray.direction.y,
                ray.origin.z + t * ray.direction.z));
        }
        if let Some(t) = self.compute_last_cylinder_surface(&origin, &center, &direction) {
            return Some(Point3D::new(
                ray.origin.x + t * ray.direction.x,
                ray.origin.y + t * ray.direction.y,
                ray.origin.z + t * ray.direction.z));
        }
        None
    }
    fn translate(&mut self, vec:Vector3D) {
        self.center.x += &vec.x;
        self.center.x += &vec.x;
        self.center.y += &vec.y;
    }
    fn rotatex(&mut self, _angle:f64) {}
    fn rotatey(&mut self, _angle:f64) {}
    fn rotatez(&mut self, _angle:f64) {}
    fn scale(&mut self, value:f64) {
        self.radius *= value;
    }
    fn suface_normal(&self, hit_point:Point3D) -> Vector3D {
        let axis_vec = match self.axis {
            'X' => Vector3D::new(1.0, 0.0, 0.0),
            'Y' => Vector3D::new(0.0, 1.0, 0.0),
            _ => Vector3D::new(0.0, 0.0, 1.0),
        };
        let hit_vec = hit_point - self.center;
        let proj = axis_vec * hit_vec.scal(&axis_vec);
        let normal_vec = hit_vec - proj;
        normal_vec.normalize()
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
            radius: self.radius,
            height: self.height,
            color: self.color,
            axis: self.axis,
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
            return Err("Missing radius".into());
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

    fn with_axis(&mut self, axis:Option<char>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if axis.is_none() {
            self.axis = 'Z';
        } else {
            self.axis = axis.unwrap();
        }
        Ok(())
    }

    fn with_height(&mut self, height:Option<f64>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if height.is_none() {
            self.height = 0.0;
        } else {
            self.height = height.unwrap();
        }
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

impl Default for Cylinder {
    fn default() -> Self {
        Cylinder {
            center: Point3D::default(),
            radius: 0.0,
            height: 0.0,
            color: Color::default(),
            axis: 'Z',
            pattern: Box::new(Solid::default()),
            reflectiveness: 0.0,
            transformation: Transformation::default()
        }
    }
}
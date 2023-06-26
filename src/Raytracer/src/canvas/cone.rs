//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Cone
//

use crate::math::{vector3d::Vector3D, point3d::Point3D};
use crate::interfaces::{Primitives, Mask};
use crate::matrix::Transformation;
use crate::ray_tracer::ray::Ray;
use crate::canvas::{
    color::Color,
    material::{
        Solid
    }
};

#[derive(Clone)]
pub struct Cone {
    pub center:Point3D,
    pub radius:f64,
    pub height:f64,
    pub color:Color,
    pub axis:char,
    pub direction:Vector3D,
    pub pattern:Box<dyn Mask>,
    pub reflectiveness:f64,
    transformation: Transformation
}

impl Cone {
    // pub fn new_config(center:Point3D, radius:f64, height: f64, color:Color, axis:char, pattern:Box<dyn Mask>) -> Self {
    //     let mut direction = Vector3D::default();
    //     if axis == 'X' {
    //         direction.x = 1.0;
    //     }
    //     if axis == 'Y' {
    //         direction.y = 1.0;
    //     }
    //     if axis == 'Z' {
    //         direction.y = 1.0;
    //     }
    //     Cone {center, radius, height, color, axis, direction, pattern, reflectiveness: 0.0}
    // }

    fn getorigin(&self, ray:Ray) -> Option<Vec<f64>> {
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

    fn getcenter(&self, _ray:Ray) -> Option<Vec<f64>> {
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

    fn getdirection(&self, ray:Ray) -> Option<Vec<f64>> {
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
}

impl Primitives for Cone {
    fn hits(&self, ray:Ray) -> Option<Point3D> {
        let origin: Vec<f64>;
        let center: Vec<f64>;
        let direction: Vec<f64>;

        match self.getorigin(ray) {
            Some(result) => origin = result,
            None => return None
        }
        match self.getcenter(ray) {
            Some(result) => center = result,
            None => return None
        }
        match self.getdirection(ray) {
            Some(result) => direction = result,
            None => return None
        }

        let a1 = origin[0] + center[0];
        let b1 = origin[1] + center[1];
        let d1 = self.height - origin[2] + center[2];

        let tan:f64 = (self.radius / self.height) * (self.radius / self.height);

        let a2 = direction[0].powf(2.0) + direction[1].powf(2.0) - tan * direction[2].powf(2.0);
        let b2 = 2.0 * a1 * direction[0] + (2.0 * b1 * direction[1]) + (2.0 * tan * d1 * direction[2]);
        let c = a1.powf(2.0) + b1.powf(2.0) - (tan * d1.powf(2.0));

        let delta:f64 = b2.powf(2.0) - 4.0 * (a2 * c);
        if delta.abs() < 0.001 {
            return None;
        }
        if delta < 0.0 {
            return None;
        }
        let t1:f64 = (-b2 - delta.sqrt()) / (2.0 * a2);
        let t2:f64 = (-b2 + delta.sqrt()) / (2.0 * a2);
        let t:f64;

        if t1 > t2 {
            t = t2;
        } else {
            t = t1;
        }

        let r = origin[2] + t * direction[2];

        if r >= center[2] && r <= center[2] + self.height {
            return Some(Point3D::new(
                origin[0] + t * direction[0],
                origin[1] + t * direction[1],
                origin[2] + t * direction[2]));
        }
        return None;
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
        let axis_vec = match self.axis {
            'X' => Vector3D::new(1.0, 0.0, 0.0),
            'Y' => Vector3D::new(0.0, 1.0, 0.0),
            _ => Vector3D::new(0.0, 0.0, 1.0),
        };
        let hit_vec = hit_point - self.center;
        let proj = axis_vec * hit_vec.scal(&axis_vec);
        let slope = self.radius / self.height;
        let height_vec = Vector3D::new(0.0, self.height, 0.0);
        let base_vec = height_vec * slope;
        let tip_vec = height_vec * -1.0;
        let base_radius_vec = base_vec * (proj.normalize() / height_vec.normalize());
        let normal_vec = if proj.normalize() <= height_vec.normalize() {
            hit_vec - base_radius_vec
        } else {
            let tip_proj = tip_vec * tip_vec.scal(&hit_vec);
            let radial_vec = hit_vec - tip_proj;
            radial_vec + tip_vec * slope
        };
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
            direction: self.direction,
            pattern: self.pattern.clone(),
            reflectiveness: self.reflectiveness,
            transformation: self.transformation
        })
    }

    fn with_center(&mut self, center:Option<Point3D>) -> std::result::Result<(), Box<dyn std::error::Error>> {
        if center.is_none() {
            return Err("Missing center".into());
        }
        if self.axis == 'X' {
            self.direction.x = 1.0;
        }
        if self.axis == 'Y' {
            self.direction.y = 1.0;
        }
        if self.axis == 'Z' {
            self.direction.y = 1.0;
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

impl Default for Cone {
    fn default() -> Self {
        Cone {
            center: Point3D::default(),
            radius: 0.0,
            height: 0.0,
            color: Color::default(),
            axis: 'Z',
            direction: Vector3D::default(),
            pattern: Box::new(Solid::default()),
            reflectiveness: 0.0,
            transformation: Transformation::default()
        }
    }
}
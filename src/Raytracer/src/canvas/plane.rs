//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// planes
//

use crate::math::{vector3d::Vector3D, point3d::Point3D};
use crate::interfaces::{Primitives, Mask};
use crate::matrix::Transformation;
use crate::ray_tracer::ray::Ray;

use super::color::Color;
use super::material::{
    Solid,
};

#[derive(Clone)]
pub struct Plane {
    pub axis:char,
    pub center:Point3D,
    pub direction:Vector3D,
    pub color:Color,
    pub pattern: Box<dyn Mask>,
    pub reflectiveness: f64,
    transformation: Transformation
}

impl Plane {
    // pub fn new_config(axis:char, position:f64, color:Color, pattern:Box<dyn Mask>) -> Self {
    //     let mut pos = Point3D::default();
    //     let mut direction = Vector3D::default();
    //     if axis == 'X' {
    //         pos.x = position;
    //         direction.x = 1.0;
    //     }
    //     if axis == 'Y' {
    //         pos.y = position;
    //         direction.y = 1.0;
    //     }
    //     if axis == 'Z' {
    //         pos.z = position;
    //         direction.y = 1.0;
    //     }
    //     Plane {axis, center:pos, direction: direction.normalize(), color, pattern, reflectiveness:0.0}
    // }
}

impl Primitives for Plane {
    fn hits(&self, ray:Ray) -> Option<Point3D>{
        let dot = ray.direction.scal(&self.direction);

        // print!("dot = {} {} ", dot, ray.direction);
        if dot.abs() > 1e-6 {
            let t = (self.center - ray.origin).scal(&self.direction) / dot;
            if t >= 0.0 {
                let inter_point = ray.origin + (ray.direction * t);
                return Some(inter_point);
            }
        }
        None
    }
    fn translate(&mut self, translate:Vector3D) {
        self.center.x += translate.x;
        self.center.y += translate.y;
        self.center.z += translate.z;
    }
    fn rotatex(&mut self, _:f64) {}
    fn rotatey(&mut self, _:f64) {}
    fn rotatez(&mut self, _:f64) {}
    fn scale(&mut self, _:f64) {}
    fn suface_normal(&self, _:Point3D) -> Vector3D {
        Vector3D::new(0.0, 1.0, 0.1)
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
        let mut pos:Point3D = Point3D::default();
        if self.axis == 'X' {
            pos.x = center.unwrap().x;
            self.direction.x = 1.0;
        }
        if self.axis == 'Y' {
            pos.y = center.unwrap().x;
            self.direction.y = 1.0;
        }
        if self.axis == 'Z' {
            pos.z = center.unwrap().x;
            self.direction.y = 1.0;
        }
        self.center = pos;
        Ok(())
    }

    fn with_radius(&mut self, _radius:Option<f64>) -> std::result::Result<(), Box<dyn std::error::Error>> {
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

impl Default for Plane {
    fn default() -> Self {
        Plane {
            axis: 'Z',
            center: Point3D::default(),
            direction: Vector3D::default(),
            color: Color::default() ,
            pattern: Box::new(Solid::default()),
            reflectiveness: 0.0,
            transformation: Transformation::default()
        }
    }
}


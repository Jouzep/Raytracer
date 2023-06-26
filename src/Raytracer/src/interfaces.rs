//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Primitives
//

use crate::canvas::color::Color;
use crate::math::point3d::Point3D;
use crate::math::vector3d::Vector3D;
use crate::ray_tracer::ray::Ray;

pub trait Primitives: Send {
    fn translate(&mut self, vec:Vector3D);
    fn rotatex(&mut self, angle:f64);
    fn rotatey(&mut self, angle:f64);
    fn rotatez(&mut self, angle:f64);
    fn scale(&mut self, value:f64);
    fn hits(&self, ray:Ray) -> Option<Point3D>;
    fn suface_normal(&self, hit_point:Point3D) -> Vector3D;
    fn get_color(&self) -> Color;
    fn get_pattern(&self) -> Box<dyn Mask>;
    fn get_reflectiveness(&self) -> f64;

    fn clone_box(&self) -> Box<dyn Primitives>;

    // setters for primitives builder
    fn with_center(&mut self, center:Option<Point3D>) -> std::result::Result<(), Box<dyn std::error::Error>>;

    fn with_radius(&mut self, radius:Option<f64>) -> std::result::Result<(), Box<dyn std::error::Error>>;

    fn with_color(&mut self, color:Option<Color>) -> std::result::Result<(), Box<dyn std::error::Error>>;

    fn with_pattern(&mut self, pattern:Option<Box<dyn Mask>>) -> std::result::Result<(), Box<dyn std::error::Error>>;

    fn with_reflectiveness(&mut self, reflectiveness:Option<f64>) -> std::result::Result<(), Box<dyn std::error::Error>>;

    fn with_axis(&mut self, axis:Option<char>) -> std::result::Result<(), Box<dyn std::error::Error>>;

    fn with_height(&mut self, height:Option<f64>) -> std::result::Result<(), Box<dyn std::error::Error>>;

    fn with_scale(&mut self, scale:Option<f64>) -> std::result::Result<(), Box<dyn std::error::Error>>;

    fn with_translation(&mut self, translation:Option<Vector3D>) -> std::result::Result<(), Box<dyn std::error::Error>>;

    fn with_rotation(&mut self, rotation:Option<Vector3D>) -> std::result::Result<(), Box<dyn std::error::Error>>;
}

impl Clone for Box<dyn Primitives> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

pub trait Mask: Send {
    fn color_at(&self, position:Point3D) -> Color;
    fn box_clone(&self) -> Box<dyn Mask>;
    fn set_color(&mut self, color:Color);
}

pub trait ILight: Send {
    fn position(&self) -> Point3D;
    fn color(&self) -> Color;
    fn intensity(&self) -> f64;
    fn direction(&self) -> Option<Vector3D>;
    fn clone_box(&self) -> Box<dyn ILight>;
}

impl Clone for Box<dyn ILight> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}
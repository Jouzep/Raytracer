//
// EPITECH PROJECT, 2023
// Raytracer
// File description:
// primitives_builder
//

use crate::math::{
    point3d::Point3D,
    vector3d::Vector3D
};
use crate::canvas::{
    color::Color,
};
use crate::interfaces::{Primitives, Mask};

pub struct PrimitivesBuilder {
    primitives:Option<Box<dyn Primitives>>,
    center:Option<Point3D>,
    radius:Option<f64>,
    color:Option<Color>,
    pattern:Option<Box<dyn Mask>>,
    reflectiveness:Option<f64>,
    axis: Option<char>,
    height: Option<f64>,
    scale:Option<f64>,
    translation:Option<Vector3D>,
    rotation:Option<Vector3D>
}


impl PrimitivesBuilder {
    pub fn new() -> Self {
        Self {
            primitives: None,
            center: None,
            radius: None,
            color: None,
            pattern: None,
            reflectiveness: None,
            axis: None,
            height: None,
            scale:None,
            translation:None,
            rotation:None
        }
    }

    pub fn with_primitives(mut self, primitives:Box<dyn Primitives>) -> Self {
        self.primitives = Some(primitives);
        self
    }

    pub fn with_center(mut self, center:Point3D) -> Self {
        self.center = Some(center);
        self
    }

    pub fn with_radius(mut self, radius:f64) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn with_color(mut self, color:Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_pattern(mut self, pattern:Box<dyn Mask>) -> Self {
        self.pattern = Some(pattern);
        self
    }

    pub fn with_reflectiveness(mut self, reflectiveness:f64) -> Self {
        self.reflectiveness = Some(reflectiveness);
        self
    }

    pub fn with_axis(mut self, axis:char) -> Self {
        self.axis = Some(axis);
        self
    }

    pub fn with_height(mut self, height:f64) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_scale(mut self, scale:f64) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn with_translation(mut self, translation:Vector3D) -> Self {
        self.translation = Some(translation);
        self
    }

    pub fn with_rotation(mut self, rotation:Vector3D) -> Self {
        self.rotation = Some(rotation);
        self
    }

    pub fn build(self) -> std::result::Result<Box<dyn Primitives>, Box<dyn std::error::Error>> {
        let mut primitives = self.primitives.ok_or("primitives not specified")?;

        primitives.with_center(self.center)?;
        primitives.with_color(self.color)?;
        primitives.with_pattern(self.pattern)?;
        primitives.with_radius(self.radius)?;
        primitives.with_reflectiveness(self.reflectiveness)?;
        primitives.with_axis(self.axis)?;
        primitives.with_height(self.height)?;
        primitives.with_translation(self.translation)?;
        primitives.with_rotation(self.rotation)?;
        primitives.with_scale(self.scale)?;

        Ok(primitives)
    }
}

//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Light
//

use crate::math::{
    point3d::Point3D,
    vector3d::Vector3D
};
use crate::interfaces::ILight;
use crate::canvas::color::Color;

#[derive(Copy, Clone)]
pub struct PointLight {
    pub origin:Point3D,
    pub color:Color,
    pub intensity:f64,
}

#[derive(Copy, Clone)]
pub struct DirectionalLight {
    pub color:Color,
    pub intensity:f64,
    pub direction:Vector3D
}

#[derive(Clone)]
pub struct Light {
    pub ambient:f64,
    pub diffuse:f64,
    pub specular:f64,
    pub lights:Vec<Box<dyn ILight>>,
}

impl Light {
    pub fn new_config(ambient:f64, diffuse:f64, specular:f64, lights:Vec<Box<dyn ILight>>) -> Self {
        Light { ambient, diffuse, specular, lights }
    }
}

impl ILight for PointLight {
    fn position(&self) -> Point3D {
        self.origin
    }

    fn color(&self) -> Color {
        self.color
    }

    fn intensity(&self) -> f64 {
        self.intensity
    }

    fn direction(&self) -> Option<Vector3D> {
        None
    }

    fn clone_box(&self) -> Box<dyn ILight> {
        Box::new(PointLight {
            origin: self.origin,
            color: self.color,
            intensity: self.intensity
        })
    }
}

impl ILight for DirectionalLight {
    fn position(&self) -> Point3D {
        Point3D::default()
    }

    fn color(&self) -> Color {
        self.color
    }

    fn intensity(&self) -> f64 {
        self.intensity
    }

    fn direction(&self) -> Option<Vector3D> {
        Some(self.direction)
    }

    fn clone_box(&self) -> Box<dyn ILight> {
        Box::new(DirectionalLight {
            color: self.color,
            intensity: self.intensity,
            direction: self.direction
        })
    }
}

impl Default for Light {
    fn default() -> Self {
        Light {
            ambient: 0.0,
            diffuse: 0.0,
            specular: 0.0,
            lights: Vec::new(),
        }
    }
}
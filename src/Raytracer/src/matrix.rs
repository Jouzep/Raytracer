//
// EPITECH PROJECT, 2023
// Raytracer
// File description:
// matrix
//

use crate::math::vector3d::Vector3D;

#[derive(Clone, Copy)]
pub struct Transformation {
    pub translation:Vector3D,
    pub rotation:Vector3D,
    pub scale:f64
}

impl Transformation {
    // pub fn new(translation: Vector3D, rotation: Vector3D, scale: f64) -> Self {
    //     Self { translation, rotation, scale }
    // }
}

impl Default for Transformation {
    fn default() -> Self {
        Self { translation: Default::default(), rotation: Default::default(), scale: 1.0 }
    }
}
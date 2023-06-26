//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Factory
//

use crate::interfaces::Primitives;
use crate::canvas::{
    sphere::Sphere,
    plane::Plane,
    cone::Cone,
    cylinder::Cylinder
};

pub struct Factory {
}

impl Factory {
    fn create_sphere() -> Sphere {
        Sphere::default()
    }

    fn create_plane() -> Plane{
        Plane::default()
    }

    fn create_cylinder() -> Cylinder{
        Cylinder::default()
    }

    fn create_cone() -> Cone {
        Cone::default()
    }

    pub fn create_primitives(name:&String) -> Option<Box<dyn Primitives>> {
        match name.as_str() {
            "spheres" => return Some(Box::new(Factory::create_sphere())),
            "planes" => return Some(Box::new(Factory::create_plane())),
            "cones" => return Some(Box::new(Factory::create_cone())),
            "cylinders" => return Some(Box::new(Factory::create_cylinder())),
            _ => return None,
        }
    }
}
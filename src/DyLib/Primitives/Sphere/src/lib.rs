//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// lib
//

#[path = "../../../../Raytracer/src/math"]
mod math {
    pub mod point3d;
    pub mod vector3d;
    pub mod formulas;
}

#[path = "../../../../Raytracer/src/interfaces"]
mod interfaces {
    pub mod primitives;
}

#[path = "../../../../Raytracer/src/ray_tracer"]
mod ray_tracer {
    pub mod ray;
}

mod sphere;

pub fn entryPoint() -> sphere::Sphere {
    return sphere::Sphere::default();
}
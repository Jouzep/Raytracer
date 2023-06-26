//
// EPITECH PROJECT, 2023
// ù
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

mod plane;

pub fn entryPoint() -> plane::Plane {
    return plane::Plane::default();
}

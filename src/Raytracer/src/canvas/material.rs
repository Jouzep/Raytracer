//
// EPITECH PROJECT, 2023
// Raytracer
// File description:
// material
//

use crate::math::{
    vector3d::Vector3D,
    point3d::Point3D
};

use crate::interfaces::{Mask, ILight};

use super::color::Color;

#[derive(Clone)]
pub struct PhongModel {
    ambient:f64,
    diffuse:f64,
    specular:f64,
}

impl PhongModel {
    pub fn new(ambient:f64, diffuse:f64, specular:f64) -> Self {
        Self {ambient, diffuse, specular}
    }

    fn light_with_direction(&self, color:Color, light_color:Color, coeff:f64) -> Color {
        let r = (color.r * self.ambient) + (light_color.r * self.diffuse * coeff);
        let g = (color.g * self.ambient) + (light_color.g * self.diffuse * coeff);
        let b = (color.b * self.ambient) + (light_color.b * self.diffuse * coeff);

        Color::new(r, g, b).max_rgb().min_rgb()
    }

    pub fn lightning(&self, color:Color, light:Box<dyn ILight>, position:Point3D, normal_v:Vector3D, is_shadow:bool) -> Color {
        let eff_color = color * light.intensity();
        let ambient:Color;
        let diffuse:Color;
        let specular:Color;

        let lightv = (light.position() - position).normalize();

        ambient = eff_color * self.ambient;
        let light_dot_normal = lightv.scal(&normal_v);
        if light_dot_normal < 0.0 {
            diffuse = Color::black(); // Black
            specular = Color::black(); // Black
        } else {
            diffuse = eff_color * self.diffuse * light_dot_normal;

            let reflectv = lightv.reflect(normal_v) * -1.0;
            let reflect_dot_eye = reflectv.scal(&normal_v);
            if reflect_dot_eye <= 0.0 {
                specular = Color::black();
            } else {
                let factor = reflect_dot_eye.powf(200.0);
                specular = color * self.specular * factor * light.intensity()
            }
        }
        let mut ret_color: Color;
        if is_shadow {
            ret_color = ambient;
        } else {
            ret_color = ambient + diffuse + specular;
        }

        let coeff;
        if !light.direction().is_none() {
            // Directional light
            coeff = light.direction().unwrap().scal(&normal_v) * (-1.0);
            // coeff = light.direction().unwrap().scal(&normal_v) * (-1.0);
            self.light_with_direction(eff_color, light.color(), coeff)
        } else {
            ret_color.max_rgb()
        }

    }
}

impl Default for PhongModel {
    fn default() -> Self {
        Self {
            ambient: 1.0,
            diffuse: 1.0,
            specular: 1.0
        }
    }
}

impl Clone for Box<dyn Mask> {
    fn clone(&self) -> Box<dyn Mask> {
        self.box_clone()
    }
}

// pub enum Pattern {
//     Solid(Solid),
//     Chessboard(Chessboard)
// }

#[derive(Copy, Clone)]
pub struct Solid {
    color: Color
}

impl Solid {
    pub fn new(color:Color) -> Self {
        Self { color }
    }
}

pub fn get_material_pattern(str: &str) -> Box<dyn Mask> {
    if str == "\"solid\"" {
        return Box::new(Solid::default());
    }
    if str == "\"chessboard\"" {
        return Box::new(Chessboard::default())
    }
    Box::new(Solid::default())
}

impl Default for Solid {
    fn default() -> Self {
        Solid { color: Color::default() }
    }
}

impl Mask for Solid {
    fn color_at(&self, _position:Point3D) -> Color {
        self.color
    }

    fn box_clone(&self) -> Box<dyn Mask> {
        Box::new(*self)
    }

    fn set_color(&mut self, color:Color) {
        self.color = color;
    }
}

#[derive(Copy, Clone)]
pub struct Chessboard {
    color_a: Color,
    color_b: Color
}

impl Default for Chessboard {
    fn default() -> Self {
        Self { color_a: Color::black(), color_b: Color::white() }
    }
}

impl Mask for Chessboard {
    fn color_at(&self, position:Point3D) -> Color {
        let x = position.x;
        let y = position.y;
        let z = position.z;

        if (x.floor() + y.floor() + z.floor()) as i64 % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }

    fn box_clone(&self) -> Box<dyn Mask> {
        Box::new(*self)
    }

    fn set_color(&mut self, _color:Color) {}
}

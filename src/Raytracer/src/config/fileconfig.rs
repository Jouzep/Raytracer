//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// FileConfig
//

use serde_json::{Value};
use crate::canvas::color::Color;
use crate::builder::primitives_builder::PrimitivesBuilder;
use crate::interfaces::{Primitives, Mask, ILight};
use crate::math::{
    point3d::Point3D,
    vector3d::Vector3D
};
use crate::canvas::{
    material
};

use crate::factory::primitives_factory::Factory;
use crate::ray_tracer::{
    camera::Camera,
    light::{
        Light,
        PointLight,
        DirectionalLight
    },
};
use crate::tools;

#[derive(Clone)]
pub struct SceneData {
    pub camera:Camera,
    pub primitives:Vec<Box<dyn Primitives>>,
    pub lights:Light
}

fn convert_string_to_json_obj(str: String) -> std::result::Result<Value, Box<dyn std::error::Error>> {
    let obj = serde_json::from_str(&str)?;
    Ok(obj)
}

fn config_cam(data:&Value) -> std::result::Result<Camera, Box<dyn std::error::Error>> {
    let width = data["camera"]["resolution"]["width"].to_string().parse::<u32>()?;

    let height = data["camera"]["resolution"]["height"].to_string().parse::<u32>()?;

    let position = Point3D::new(
        data["camera"]["position"]["x"].to_string().parse::<f64>()?,
        data["camera"]["position"]["y"].to_string().parse::<f64>()?,
        data["camera"]["position"]["z"].to_string().parse::<f64>()?);

    let rotation = Vector3D::new(
        data["camera"]["rotation"]["x"].to_string().parse::<f64>()?,
        data["camera"]["rotation"]["x"].to_string().parse::<f64>()?,
        data["camera"]["rotation"]["x"].to_string().parse::<f64>()?);

    let fov = data["camera"]["fieldOfView"].to_string().parse::<f64>()?;

    Ok(Camera::new_config(width, height, position, rotation, fov))
}

fn config_lights(data:&Value) -> std::result::Result<Light, Box<dyn std::error::Error>> {
    let ambient = data["lights"]["ambient"].to_string().parse::<f64>()?;
    let diffuse = data["lights"]["diffuse"].to_string().parse::<f64>()?;
    let specular = data["lights"]["specular"].to_string().parse::<f64>()?;
    let points_len =  data["lights"]["point"]
    .as_array()
    .ok_or("Not an array")?.len();
    let directionals_len =  data["lights"]["directional"]
    .as_array()
    .ok_or("Not an array")?.len();
    let mut lights: Vec<Box<dyn ILight>> = Vec::new();

    for i in 0..points_len {
        let point = Point3D::new(
            data["lights"]["point"][i]["x"].to_string().parse::<f64>()?,
            data["lights"]["point"][i]["y"].to_string().parse::<f64>()?,
            data["lights"]["point"][i]["z"].to_string().parse::<f64>()?);

        let mut color = Color::white();
        if !data["lights"]["point"][i]["color"].is_null() {
            color = Color::new(
                data["lights"]["point"][i]["color"]["x"].to_string().parse::<f64>()?,
                data["lights"]["point"][i]["color"]["y"].to_string().parse::<f64>()?,
                data["lights"]["point"][i]["color"]["z"].to_string().parse::<f64>()?);
        }
        let intensity;
        if data["lights"]["point"][i]["intensity"].is_null() {
            intensity = 1.0
        } else {
            intensity = data["lights"]["point"][i]["intensity"].to_string().parse::<f64>()?;
        }
        lights.push(Box::new(PointLight { origin: point, color, intensity }));
    }

    for i in 0..directionals_len {
        let color = Color::new(
            data["lights"]["directional"][i]["color"]["r"].to_string().parse::<f64>()?,
            data["lights"]["directional"][i]["color"]["g"].to_string().parse::<f64>()?,
            data["lights"]["directional"][i]["color"]["b"].to_string().parse::<f64>()?);

        let intensity;
        if data["lights"]["point"][i]["intensity"].is_null() {
            intensity = 1.0;
        } else {
            intensity = data["lights"]["point"][i]["intensity"].to_string().parse::<f64>()?;
        }

        let direction = Vector3D::new(
            data["lights"]["directional"][i]["rotation"]["x"].to_string().parse::<f64>()?,
            data["lights"]["directional"][i]["rotation"]["y"].to_string().parse::<f64>()?,
            data["lights"]["directional"][i]["rotation"]["z"].to_string().parse::<f64>()?);

        lights.push(Box::new(DirectionalLight { color, intensity, direction }))
    }
    Ok(Light::new_config(ambient, diffuse, specular, lights))
}

fn get_primitives_keys(data:&Value) -> Vec<String> {
    let mut new: Vec<String> = Vec::new();
    if let Some(obj) = data["primitives"].as_object() {
        for key in obj.keys() {
            new.push(key.clone());
        }
    }
    new
}

fn config_prims(data:&Value) -> std::result::Result<Vec<Box<dyn Primitives>>, Box<dyn std::error::Error>> {
    let mut primitives: Vec<Box<dyn Primitives>> = Vec::new();

    let prims_key: Vec<String> = get_primitives_keys(data);

    // Iterate into each primitive's keys
    for prims in prims_key {

        // Get the len of the primitive's array
        let prims_len = data["primitives"][&prims]
        .as_array()
        .map_or(0, |arr| arr.len());

        // iterate in each array of primitives
        for i in 0..prims_len {

            //** Must Values **//

            // Get the position
            let position: Point3D;
            if !data["primitives"][&prims][i]["position"].is_null() {
                position = Point3D::new(
                    data["primitives"][&prims][i]["position"].to_string().parse::<f64>()?,
                    0.0,
                    0.0);
            } else {
                position = Point3D::new(
                data["primitives"][&prims][i]["x"].to_string().parse::<f64>()?,
                data["primitives"][&prims][i]["y"].to_string().parse::<f64>()?,
                data["primitives"][&prims][i]["z"].to_string().parse::<f64>()?);
            }

            // Get the color
            let color = Color::new(
                data["primitives"][&prims][i]["color"]["r"].to_string().parse::<f64>()?,
                data["primitives"][&prims][i]["color"]["g"].to_string().parse::<f64>()?,
                data["primitives"][&prims][i]["color"]["b"].to_string().parse::<f64>()?);

            //** End Must Values **//

            //** Optional Values **//

            // Get the radius
            let mut radius = 0.0;
            if !data["primitives"][&prims][i]["r"].is_null() {
                radius = data["primitives"][&prims][i]["r"].to_string().parse::<f64>()?;
            }

            // Set the color
            let mut pattern: Box<dyn Mask> = Box::new(material::Solid::new(color));
            if !data["primitives"][&prims][i]["material"]["pattern"].is_null() {
                let pattern_str = data["primitives"][&prims][i]["material"]["pattern"].to_string().parse::<String>()?;
                pattern = material::get_material_pattern(pattern_str.as_str());
            }
            pattern.set_color(color);

            // Set the reflectiveness
            let mut reflectiveness:f64 = 0.0;
            if !data["primitives"][&prims][i]["material"]["reflectiveness"].is_null() {
                reflectiveness = data["primitives"][&prims][i]["material"]["reflectiveness"].to_string().parse::<f64>()?;
            }

            // Set the axis
            let mut axis = 'Z';
            if !data["primitives"][&prims][i]["axis"].is_null() {
                let axis_str = data["primitives"][&prims][i]["axis"].to_string().parse::<String>()?;
                axis = axis_str[1..2].chars().next().unwrap();
            }

            // Set the height
            let mut height = 0.0;
            if !data["primitives"][&prims][i]["h"].is_null() {
                height = data["primitives"][&prims][i]["h"].to_string().parse::<f64>()?;
            }

            // Set the scale
            let mut scale = 1.0;
            if !data["primitives"][&prims][i]["transform"]["scale"].is_null() {
                scale = data["primitives"][&prims][i]["transform"]["scale"].to_string().parse::<f64>()?;
            }

            let mut translation = Vector3D::default();
            if !data["primitives"][&prims][i]["transform"]["translation"].is_null() {
                translation = Vector3D::new(
                    data["primitives"][&prims][i]["transform"]["translation"]["x"].to_string().parse::<f64>()?,
                    data["primitives"][&prims][i]["transform"]["translation"]["y"].to_string().parse::<f64>()?,
                    data["primitives"][&prims][i]["transform"]["translation"]["z"].to_string().parse::<f64>()?);
            }

            let mut rotation = Vector3D::default();
            if !data["primitives"][&prims][i]["transform"]["rotation"].is_null() {
                rotation = Vector3D::new(
                    data["primitives"][&prims][i]["transform"]["rotation"]["x"].to_string().parse::<f64>()?,
                    data["primitives"][&prims][i]["transform"]["rotation"]["y"].to_string().parse::<f64>()?,
                    data["primitives"][&prims][i]["transform"]["rotation"]["z"].to_string().parse::<f64>()?);
            }

            //** End Optional Values **//

            let mut new = PrimitivesBuilder::new()
                .with_primitives(Factory::create_primitives(&prims).unwrap())
                .with_axis(axis)
                .with_center(position)
                .with_color(color)
                .with_pattern(pattern)
                .with_radius(radius)
                .with_reflectiveness(reflectiveness)
                .with_height(height)
                .with_translation(translation)
                .with_rotation(rotation)
                .with_scale(scale)
                .build()?;

            new.translate(translation);
            new.rotatex(rotation.x);
            new.rotatey(rotation.y);
            new.rotatez(rotation.z);
            new.scale(scale);

            primitives.push(new);
        }
    }
    Ok(primitives)
}

impl SceneData {
    pub fn new(filepath: &str) -> std::result::Result<SceneData, Box<dyn std::error::Error>> {
        // Convert string to json
        let data = convert_string_to_json_obj(tools::read_file(&filepath)?)?;

        // Get camera's configs
        let camera = config_cam(&data)?;

        // Get primitives's configs
        let primitives = config_prims(&data)?;

        // Get lights's configs
        let lights = config_lights(&data)?;

        Ok(SceneData { camera, primitives, lights})
    }
}

mod error_handler;
mod usage;
mod raytracer;
mod math {
    pub mod point3d;
    pub mod vector3d;
    pub mod formulas;
}

mod tools;

mod builder {
    pub mod primitives_builder;
}

mod canvas {
    pub mod color;
    pub mod sphere;
    pub mod plane;
    pub mod cylinder;
    pub mod cone;
    pub mod material;
}

mod ray_tracer {
    pub mod ray;
    pub mod camera;
    pub mod rectangle3d;
    pub mod light;
}

mod matrix;

mod interfaces;

mod config {
    pub mod fileconfig;
}

mod factory {
    pub mod primitives_factory;
}

fn main() -> std::process::ExitCode {
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 2 && &args[1] == "--help" {
        usage::display_usage(&args[0]);
        return std::process::ExitCode::SUCCESS;
    }
    let (status, file_index): (u32, usize) = match error_handler::error_handler(&args) {
        (1, _) => return std::process::ExitCode::from(84),
        value => value
    };

    let scene = config::fileconfig::SceneData::new(&args[file_index]);

    match scene {
        Ok(s) => {
            // let mutable_scene = &mut s;
            if status == 0 {
                raytracer::run_raytracer(s);
            } else if status == 2 {
                raytracer::run_raytracer_multithreading(s);
            }
            return std::process::ExitCode::SUCCESS;
        },
        Err(_) => {
            eprintln!("Error in reading the scene config");
            return std::process::ExitCode::from(84)
        }
    }
}
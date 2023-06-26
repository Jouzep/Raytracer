//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// formulas
//

use crate::math::{point3d::Point3D, vector3d::Vector3D};

pub fn compute_discriminant(a:f64, b:f64, c:f64) -> f64 {
    return b.powf(2.0) - 4.0 * a * c;
}

pub fn resolve_quadratic_eq(delta:f64, a:f64, b:f64) -> Option<Vec<f64>>{
    let mut res: Vec<f64> = Vec::new();
    if delta < 0.0 {
        return None;
    }
    if delta == 0.0 {
        res.push(-b / 2.0 * a );
        return Some(res);
    }

    res.push((-b + delta.sqrt()) / (2.0 * a));
    res.push((-b - delta.sqrt()) / (2.0 * a));

    return Some(res);
}

pub fn get_inter_point_from_eq(eqs:Vec<f64>, point:Point3D, direction:Vector3D) -> Vec<Point3D>{
    let mut res: Vec<Point3D> = Vec::new();
    if eqs.len() == 1 {
        res.push(Point3D::new(point.x + eqs[0] * direction.x, point.y + eqs[0] * direction.y, point.z + eqs[0] * direction.z));
        return res;
    }
    if eqs.len() == 2 {
        res.push(Point3D::new(point.x + eqs[0] * direction.x, point.y + eqs[0] * direction.y, point.z + eqs[0] * direction.z));
        res.push(Point3D::new(point.x + eqs[1] * direction.x, point.y + eqs[1] * direction.y, point.z + eqs[1] * direction.z));
        return res;
    }
    return res;
}

pub fn get_closest_point(hit_points:Vec<Point3D>, origin:Point3D) -> Point3D {
    if hit_points.len() == 1 {
        return  hit_points[0];
    } else {
        let p1 = (hit_points[0] - origin).abs();
        let p2 = (hit_points[1] - origin).abs();
        let p1_tot = p1.x + p1.z + p1.z;
        let p2_tot = p2.x + p2.z + p2.z;
        if p1_tot > p2_tot {
            return hit_points[1];
        } else {
            return hit_points[0];
        }
    }
}

pub fn _suface_normal_vector(hit_point:Vector3D) -> Vector3D {
    let norme = (hit_point.x * hit_point.x + hit_point.y * hit_point.y + hit_point.z * hit_point.z).sqrt();
    return Vector3D::new(hit_point.x / norme, hit_point.y / norme, hit_point.z / norme)
}

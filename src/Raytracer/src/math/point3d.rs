//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Point3D
//

use crate::math::vector3d::Vector3D;

#[derive(Copy, Clone, PartialEq)]
pub struct Point3D {
    pub x:f64,
    pub y:f64,
    pub z:f64
}

impl Point3D {
    pub fn new(x:f64, y:f64, z:f64) -> Point3D {
        return Point3D {x, y, z};
    }

    pub fn abs(&self) -> Point3D {
        return  Point3D {x: self.x.abs(), y: self.y.abs(), z: self.z.abs()};
    }
}

impl Default for Point3D {
    fn default() -> Self {
        Point3D {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}

impl std::ops::Add<Vector3D> for Point3D {
    type Output = Point3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        return Point3D {x:self.x + rhs.x , y:self.y + rhs.y, z:self.z + rhs.z};
    }
}

impl std::ops::Sub<Vector3D> for Point3D {
    type Output = Point3D;

    fn sub(self, rhs: Vector3D) -> Self::Output {
        return Point3D {x:self.x - rhs.x , y:self.y - rhs.y, z:self.z - rhs.z};
    }
}

impl std::ops::Mul<Vector3D> for Point3D {
    type Output = Point3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        return Point3D {x:self.x * rhs.x , y:self.y * rhs.y, z:self.z * rhs.z};
    }
}

impl std::ops::Mul<f64> for Point3D {
    type Output = Point3D;

    fn mul(self, rhs:f64) -> Self::Output {
        return Point3D {x:self.x * rhs, y:self.y * rhs, z:self.z * rhs}
    }
}

// impl std::ops::Mul<Vector3D> for Point3D {
//     type Output = Point3D;

//     fn mul(self, rhs: Vector3D) -> Self::Output {
//         return Point3D {x:self.x * rhs.x , y:self.y * rhs.y, z:self.z * rhs.z};
//     }
// }

impl std::ops::Div<Vector3D> for Point3D {
    type Output = Point3D;

    fn div(self, rhs: Vector3D) -> Self::Output {
        return Point3D {x:self.x / rhs.x , y:self.y / rhs.y, z:self.z / rhs.z};
    }
}

impl std::ops::Sub<Point3D> for Point3D{
    type Output = Vector3D;
    fn sub(self, rhs: Point3D) -> Self::Output {
        Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z
        }
    }
}

impl std::fmt::Display for Point3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Point3D(x={}, y={}, z={})", self.x, self.y, self.z)
    }
}
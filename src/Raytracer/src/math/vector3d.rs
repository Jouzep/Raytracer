//
// EPITECH PROJECT, 2023
// B-OOP-400-PAR-4-1-raytracer-vincent.shao
// File description:
// Vector3D
//

#[derive(Copy, Clone)]
pub struct Vector3D {
    pub x:f64,
    pub y:f64,
    pub z:f64,
}

impl Vector3D {
    pub fn new(x:f64, y:f64, z:f64) -> Vector3D {
        return Vector3D {x, y, z};
    }

    pub fn scal(&self, other: &Self) -> f64 {
        return (self.x * other.x) + (self.y * other.y) + (self.z * other.z);
    }

    pub fn length(&self) -> f64 {
        let res: f64 = (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();
        return res;
    }

    pub fn cross(&self, other:&Self) -> Self {
        let new_x = self.y * other.z - other.y - self.z;
        let new_y = -self.x * other.z - other.x * self.z;
        let new_z = self.x * other.y - other.x - self.y;
        return Vector3D::new(new_x, new_y, new_z);
    }

    pub fn abs(&self) -> Vector3D {
        return  Vector3D {x: self.x.abs(), y: self.y.abs(), z: self.z.abs()};
    }

    pub fn normalize(&self) -> Vector3D {
        let n = (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();
        return Vector3D::new(self.x / n, self.y / n, self.z / n);
    }

    pub fn normalize_squared(&self) -> f64 {
        self.scal(&self)
    }

    pub fn reflect(&self, other:Vector3D) -> Vector3D {
        *self - other * 2.0 * self.scal(&other)
        // let normal = other / other.normalize();
        // let reflected_vector = *self - normal * 2.0 * (*self).scal(&normal);
        // reflected_vector
    }
}

impl Default for Vector3D {
    fn default() -> Self {
        Vector3D { x: (0.0), y: (0.0), z: (0.0) }
    }
}

impl std::ops::Add<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Vector3D) -> Self::Output {
        return Vector3D {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z};
    }
}

impl std::ops::AddAssign<Vector3D> for Vector3D {
    fn add_assign(&mut self, rhs: Vector3D) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Vector3D) -> Self::Output {
        return Vector3D {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z};
    }
}

impl std::ops::Sub<f64> for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: f64) -> Self::Output {
        return Vector3D {x: self.x - rhs, y: self.y - rhs, z: self.z - rhs};
    }
}

impl std::ops::Sub<Vector3D> for f64 {
    type Output = Vector3D;

    fn sub(self, rhs: Vector3D) -> Self::Output {
        return Vector3D {x: self - rhs.x, y: self - rhs.y, z: self - rhs.z};
    }
}

impl std::ops::SubAssign<Vector3D> for Vector3D {
    fn sub_assign(&mut self, rhs: Vector3D) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Self::Output {
        return Vector3D {x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z};
    }
}

impl std::ops::Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        return Vector3D {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs};
    }
}

impl std::ops::MulAssign<Vector3D> for Vector3D {
    fn mul_assign(&mut self, rhs: Vector3D) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl std::ops::Div<Vector3D> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: Vector3D) -> Self::Output {
        return Vector3D {x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z};
    }
}

impl std::ops::Div<f64> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f64) -> Self::Output {
        return Vector3D {x: self.x / rhs, y: self.y / rhs, z: self.z / rhs};
    }
}

impl std::ops::DivAssign<Vector3D> for Vector3D {
    fn div_assign(&mut self, rhs: Vector3D) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl std::fmt::Display for Vector3D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector3D(x={}, y={}, z={})", self.x, self.y, self.z)
    }
}

impl PartialOrd for Vector3D {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x > other.x && self.y > other.y && self.z > other.z {
            Some(std::cmp::Ordering::Greater)
        } else if self.x >= other.x && self.y >= other.y && self.z >= other.z {
            Some(std::cmp::Ordering::Greater)
        } else {
            None
        }
    }
}

impl PartialEq for Vector3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == self.y && self.z == other.z
    }
}
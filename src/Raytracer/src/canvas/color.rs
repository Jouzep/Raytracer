//
// EPITECH PROJECT, 2023
// Raytracer
// File description:
// color
//

#[derive(Copy, Clone)]
pub struct Color {
    pub r:f64,
    pub g:f64,
    pub b:f64
}

impl Color {
    pub fn new(r:f64, g:f64, b:f64) -> Self {
        Self {r, g, b}
    }

    pub fn black() -> Self {
        Self { r: 0.0, g: 0.0, b: 0.0 }
    }

    pub fn white() -> Self {
        Self { r: 255.0, g: 255.0, b: 255.0 }
    }

    pub fn tot_color_value(&self) -> f64 {
        self.r + self.g + self.b
    }

    pub fn max_rgb(&mut self) -> Self {
        if self.r >= 255.0 {
            self.r = 255.0;
        }
        if self.g >= 255.0 {
            self.g = 255.0;
        }
        if self.b >= 255.0 {
            self.b = 255.0;
        }
        *self
    }

    pub fn min_rgb(&mut self) -> Self {
        if self.r < 0.0 {
            self.r = 0.0;
        }
        if self.g < 0.0 {
            self.g = 0.0;
        }
        if self.b < 0.0 {
            self.b = 0.0;
        }
        *self
    }

}

impl Default for Color {
    fn default() -> Self {
        Self::black()
    }
}

impl std::ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        return Color {r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b};
    }
}

impl std::ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl std::ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        return Color {r: self.r - rhs.r, g: self.g - rhs.g, b: self.b - rhs.b};
    }
}

impl std::ops::Sub<f64> for Color {
    type Output = Color;

    fn sub(self, rhs: f64) -> Self::Output {
        return Color {r: self.r - rhs, g: self.g - rhs, b: self.b - rhs};
    }
}

impl std::ops::Sub<Color> for f64 {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        return Color {r: self - rhs.r, g: self - rhs.g, b: self - rhs.b};
    }
}

impl std::ops::SubAssign<Color> for Color {
    fn sub_assign(&mut self, rhs: Color) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        return Color {r: self.r * rhs.r, g: self.g * rhs.g, b: self.b * rhs.b};
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        return Color {r: self.r * rhs, g: self.g * rhs, b: self.b * rhs};
    }
}

impl std::ops::MulAssign<Color> for Color {
    fn mul_assign(&mut self, rhs: Color) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
    }
}

impl std::ops::Div<Color> for Color {
    type Output = Color;

    fn div(self, rhs: Color) -> Self::Output {
        return Color {r: self.r / rhs.r, g: self.g / rhs.g, b: self.b / rhs.b};
    }
}

impl std::ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        return Color {r: self.r / rhs, g: self.g / rhs, b: self.b / rhs};
    }
}

impl std::ops::DivAssign<Color> for Color {
    fn div_assign(&mut self, rhs: Color) {
        self.r /= rhs.r;
        self.g /= rhs.g;
        self.b /= rhs.b;
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Color(x={}, y={}, z={})", self.r, self.g, self.b)
    }
}

impl PartialOrd for Color {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.r > other.r && self.g > other.g && self.b > other.b {
            Some(std::cmp::Ordering::Greater)
        } else if self.r >= other.r && self.g >= other.g && self.b >= other.b {
            Some(std::cmp::Ordering::Greater)
        } else {
            None
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == self.g && self.b == other.b
    }
}
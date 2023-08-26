#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    pub fn clamp(self) -> Color {
        Color::new(
            self.red.clamp(0.0, 1.0),
            self.green.clamp(0.0, 1.0),
            self.blue.clamp(0.0, 1.0),
        )
    }

    pub fn out(self) -> [u8; 3] {
        [
            (self.red * 255.0) as u8,
            (self.green * 255.0) as u8,
            (self.blue * 255.0) as u8,
        ]
    }

    pub fn blend(&self, other: &Color) -> Color {
        Color::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }

    pub fn mul(&self, x: f64) -> Color {
        Color::new(self.red * x, self.blue * x, self.green * x)
    }

    pub fn by_name(name: &str) -> Color {
        let (red, green, blue) = match name {
            "black" => (0.0, 0.0, 0.0),
            "white" => (1.0, 1.0, 1.0),
            "red" => (1.0, 0.0, 0.0),
            "green" => (0.0, 1.0, 0.),
            "blue" => (0.0, 0.0, 1.0),
            _ => panic!("color not found."),
        };
        Color::new(red, green, blue)
    }

    pub fn average(colors: Vec<Color>) -> Color {
        if colors.is_empty() {
            return Color::by_name("black");
        }

        let vec_length = colors.len() as f64;
        let mut resulting_color = Color::new(0.0, 0.0, 0.0);
        for color in colors {
            resulting_color = resulting_color + color
        }
        (resulting_color / vec_length).clamp()
    }
}

impl std::ops::Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl std::ops::Mul for Color {
    type Output = Color;
    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

impl std::ops::Div<f64> for Color {
    type Output = Color;
    fn div(self, rhs: f64) -> Self::Output {
        Color::new(self.red / rhs, self.green / rhs, self.blue / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_color_clamp() {
        let test_color = Color::new(12.43, 354.0, 0.34);

        let control = Color::new(1.0, 1.0, 0.34);

        assert_eq!(control, test_color.clamp())
    }

    #[test]
    fn test_color_add() {
        let test_color1 = Color::new(0.684, 0.12, 0.238);
        let test_color2 = Color::new(0.353, 0.23, 0.0);

        let control_color = Color::new(1.037, 0.35, 0.238);

        assert_eq!(control_color, test_color1 + test_color2)
    }
}

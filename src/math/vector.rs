use rand::Rng;
use std::ops::{Add, Div, Mul, Neg, Sub};

/// A 3D vector with `x`, `y`, and `z` components.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector3 {
    /// The x-coordinate of the vector.
    pub x: f64,
    /// The y-coordinate of the vector.
    pub y: f64,
    /// The z-coordinate of the vector.
    pub z: f64,
}

impl Vector3 {
    /// Creates a new `Vector3` instance with the specified `x`, `y`, and `z` components,
    /// and then normalizes the vector.
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector3 { x, y, z }
    }

    /// Returns a normalized copy of the vector, i.e., a vector with the same direction but unit length.
    pub fn normalized(self) -> Self {
        self / self.length()
    }

    /// Returns the cross product of two vectors `self` and `other`.
    pub fn cross(self, other: Self) -> Self {
        Vector3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// Returns the dot product of two vectors `self` and `other`.
    pub fn dot(self, other: Vector3) -> f64 {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }

    /// Multiplies each component of the vector `self` by the given `scalar`.
    pub fn mul(self, rhs: f64) -> Self {
        self * rhs
    }

    /// Divides each component of the vector `self` by the given `scalar`.
    pub fn div(self, rhs: f64) -> Self {
        self / rhs
    }

    /// Adds the vector `other` to `self`.
    pub fn add(self, rhs: Self) -> Self {
        self + rhs
    }

    /// Subtracts the vector `other` from `self`.
    pub fn sub(self, rhs: Self) -> Self {
        self - rhs
    }

    /// Returns the negation of the vector `self`.
    pub fn neg(self) -> Self {
        -self
    }

    /// Returns the length of the vector.
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    /// Returns the squared length of the vector.
    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    /// Returns the angle (in radians) between two normalized vectors `a` and `b`.
    pub fn theta(a: Vector3, b: Vector3) -> f64 {
        a.normalized().dot(b.normalized())
    }

    /// Returns the distance between the vector `self` and another vector `other`.
    pub fn distance_to(&self, other: Vector3) -> f64 {
        ((other.x - self.x).powi(2) + (other.y - self.y).powi(2) + (other.z - self.z).powi(2))
            .sqrt()
    }

    /// Generates a random 3D vector within the specified range `[min, max]`.
    pub fn random(min: f64, max: f64) -> Vector3 {
        let mut rng = rand::thread_rng();
        Vector3::new(
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
            rng.gen_range(min..=max),
        )
    }

    /// Generates a random 3D unit vector (vector with unit length).
    pub fn random_unit() -> Vector3 {
        Vector3::random(-1.0, 1.0).normalized()
    }

    /// Generates a random 3D vector within a unit sphere.
    pub fn random_in_unit_sphere() -> Vector3 {
        loop {
            let temp_vector = Vector3::random(-1.0, 1.0);
            if temp_vector.length() < 1.0 {
                break temp_vector;
            }
        }
    }

    /// Reflects the incident vector `i` off a surface with the given normal vector `n`.
    pub fn reflect(i: Vector3, n: Vector3) -> Vector3 {
        i - n * i.dot(n) * 2.0
    }
}

impl Neg for Vector3 {
    type Output = Vector3;

    /// Returns the negation of the vector `self`.
    fn neg(self) -> Self {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    /// Adds the vector `other` to `self`.
    fn add(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    /// Subtracts the vector `other` from `self`.
    fn sub(self, rhs: Self) -> Self::Output {
        Vector3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    /// Multiplies each component of the vector `self` by the given `scalar`.
    fn mul(self, rhs: f64) -> Self::Output {
        Vector3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    /// Multiplies each component of the vector `rhs` by the given `scalar`.
    fn mul(self, rhs: Vector3) -> Self::Output {
        Vector3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl Div<f64> for Vector3 {
    type Output = Vector3;

    /// Divides each component of the vector `self` by the given `scalar`.
    fn div(self, rhs: f64) -> Self::Output {
        let scalar = 1.0 / rhs;
        self * scalar
    }
}

/// A 2D vector with `x` and `y` components.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    /// Creates a new `Vector2` instance with the specified `x` and `y` components.
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }

    /// Returns the length of the vector.
    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns the dot product of two vectors `self` and `other`.
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y
    }

    /// Adds the vector `other` to `self`.
    pub fn add(self, other: Self) -> Self {
        Vector2::new(self.x + other.x, self.y + other.y)
    }

    /// Subtracts the vector `other` from `self`.
    pub fn sub(self, other: Self) -> Self {
        Vector2::new(self.x - other.x, self.y - other.y)
    }

    /// Multiplies each component of the vector `self` by the given `scalar`.
    pub fn mul(self, scalar: f64) -> Self {
        Vector2::new(self.x * scalar, self.y * scalar)
    }

    /// Divides each component of the vector `self` by the given `scalar`.
    pub fn div(self, scalar: f64) -> Self {
        Vector2::new(self.x / scalar, self.y / scalar)
    }

    /// Returns a normalized copy of the vector, i.e., a vector with the same direction but unit length.
    pub fn normalize(self) -> Self {
        let length = self.length();
        if length != 0.0 {
            Vector2::new(self.x / length, self.y / length)
        } else {
            self
        }
    }
}

impl Add<Vector2> for Vector2 {
    type Output = Vector2;

    /// Adds the vector `other` to `self`.
    fn add(self, other: Vector2) -> Vector2 {
        Vector2::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub<Vector2> for Vector2 {
    type Output = Vector2;

    /// Subtracts the vector `other` from `self`.
    fn sub(self, other: Vector2) -> Vector2 {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f64> for Vector2 {
    type Output = Vector2;

    /// Multiplies each component of the vector `self` by the given `scalar`.
    fn mul(self, scalar: f64) -> Vector2 {
        Vector2::new(self.x * scalar, self.y * scalar)
    }
}

impl Div<f64> for Vector2 {
    type Output = Vector2;

    /// Divides each component of the vector `self` by the given `scalar`.
    fn div(self, scalar: f64) -> Vector2 {
        Vector2::new(self.x / scalar, self.y / scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 1e-10; // for floating point comparison

    // Helper function to compare floats with an epsilon
    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    // Vector3 Tests
    #[test]
    fn test_vector3_new() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector3_normalized() {
        let v = Vector3::new(0.0, 3.0, 4.0);
        let normalized_v = v.normalized();
        assert!(approx_eq(normalized_v.length(), 1.0));
    }

    #[test]
    fn test_vector3_cross_product() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        let cross_product = v1.cross(v2);
        assert_eq!(cross_product, Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_vector3_dot_product() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(2.0, 3.0, 4.0);
        assert!(approx_eq(v1.dot(v2), 20.0));
    }

    #[test]
    fn test_vector3_mul() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let result = v.mul(2.0);
        assert_eq!(result, Vector3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_vector3_div() {
        let v = Vector3::new(2.0, 4.0, 6.0);
        let result = v.div(2.0);
        assert_eq!(result, Vector3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_vector3_add() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(2.0, 3.0, 4.0);
        let result = v1.add(v2);
        assert_eq!(result, Vector3::new(3.0, 5.0, 7.0));
    }

    #[test]
    fn test_vector3_sub() {
        let v1 = Vector3::new(3.0, 5.0, 7.0);
        let v2 = Vector3::new(2.0, 3.0, 4.0);
        let result = v1.sub(v2);
        assert_eq!(result, Vector3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_vector3_neg() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let result = v.neg();
        assert_eq!(result, Vector3::new(-1.0, -2.0, -3.0));
    }

    #[test]
    fn test_vector3_length() {
        let v = Vector3::new(1.0, 2.0, 2.0);
        assert!(approx_eq(v.length(), 3.0));
    }

    #[test]
    fn test_vector3_length_squared() {
        let v = Vector3::new(1.0, 2.0, 2.0);
        assert!(approx_eq(v.length_squared(), 9.0));
    }

    #[test]
    fn test_vector3_theta() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(1.0, 0.0, 0.0);
        let angle = Vector3::theta(v1, v2);
        assert!(approx_eq(angle, 1.0)); // dot product of two unit vectors pointing in the same direction is 1
    }

    #[test]
    fn test_vector3_distance_to() {
        let v1 = Vector3::new(1.0, 1.0, 1.0);
        let v2 = Vector3::new(2.0, 2.0, 2.0);
        let distance = v1.distance_to(v2);
        assert!(approx_eq(distance, (3.0 as f64).sqrt()));
    }

    #[test]
    fn test_vector3_random() {
        let v = Vector3::random(0.0, 1.0);
        assert!(v.x >= 0.0 && v.x <= 1.0);
        assert!(v.y >= 0.0 && v.y <= 1.0);
        assert!(v.z >= 0.0 && v.z <= 1.0);
    }

    #[test]
    fn test_vector3_random_unit() {
        let v = Vector3::random_unit();
        assert!(approx_eq(v.length(), 1.0));
    }

    #[test]
    fn test_vector3_random_in_unit_sphere() {
        let v = Vector3::random_in_unit_sphere();
        assert!(v.length_squared() <= 1.0);
    }

    // Vector2 Tests
    #[test]
    fn test_vector2_new() {
        let v = Vector2::new(1.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn test_vector2_length() {
        let v = Vector2::new(3.0, 4.0);
        assert!(approx_eq(v.length(), 5.0));
    }

    #[test]
    fn test_vector2_dot_product() {
        let v1 = Vector2::new(1.0, 2.0);
        let v2 = Vector2::new(3.0, 4.0);
        assert!(approx_eq(v1.dot(v2), 11.0));
    }
}

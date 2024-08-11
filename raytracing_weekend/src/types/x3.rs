use std::fmt::Display;

#[derive(Clone, Copy)]
pub struct X3(pub f64, pub f64, pub f64);

impl X3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn to_unit_vec(self) -> Self {
        self / self.length()
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn r(&self) -> f64 {
        self.0
    }

    pub fn g(&self) -> f64 {
        self.1
    }

    pub fn b(&self) -> f64 {
        self.2
    }

    pub fn dot(&self, other: &X3) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }
}

impl std::ops::Add<X3> for X3 {
    type Output = X3;

    fn add(self, rhs: X3) -> Self::Output {
        X3::new(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Sub<X3> for X3 {
    type Output = X3;

    fn sub(self, rhs: X3) -> Self::Output {
        X3::new(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl std::ops::Mul<i32> for X3 {
    type Output = X3;

    fn mul(self, rhs: i32) -> Self::Output {
        let rhs = rhs as f64;
        X3::new(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::Mul<f64> for X3 {
    type Output = X3;

    fn mul(self, rhs: f64) -> Self::Output {
        X3::new(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl std::ops::Div<f64> for X3 {
    type Output = X3;

    fn div(self, rhs: f64) -> Self::Output {
        X3::new(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl std::ops::Neg for X3 {
    type Output = X3;

    fn neg(self) -> Self::Output {
        X3::new(-self.0, -self.1, -self.2)
    }
}

impl Display for X3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:3} {:3} {:3}", self.0, self.1, self.2)
    }
}

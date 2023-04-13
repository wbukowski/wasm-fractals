use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    pub real: f64,
    pub imaginary: f64,
}

impl ops::Add<Complex> for Complex {
    type Output = Complex;
    fn add(self, c: Complex) -> Complex {
        Complex {
            real: self.real + c.real,
            imaginary: self.imaginary + c.imaginary,
        }
    }
}

impl ops::Sub<Complex> for Complex {
    type Output = Complex;
    fn sub(self, c: Complex) -> Complex {
        Complex {
            real: self.real - c.real,
            imaginary: self.imaginary - c.imaginary,
        }
    }
}

impl ops::Mul<Complex> for Complex {
    type Output = Complex;
    fn mul(self, c: Complex) -> Complex {
        Complex {
            real: self.real*c.real - self.imaginary*c.imaginary,
            imaginary: self.real*c.imaginary + self.imaginary*c.real,
        }
    }
}
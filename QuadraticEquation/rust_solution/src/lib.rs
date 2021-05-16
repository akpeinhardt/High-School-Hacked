#[derive(Debug)]
struct Quadratic {
    x_squared_coef: f64,
    x_coef: f64,
    constant: f64
}

#[derive(Debug, PartialEq)]
enum QuadraticResult {
    TwoRealRoots((f64, f64)),
    OneRealRoot(f64),
    ImaginaryRoots
}

impl Quadratic {
    fn new(x_squared_coef: f64, x_coef: f64, constant: f64) -> Self {
        Self {
            x_squared_coef,
            x_coef,
            constant
        }
    }

    fn calculate_roots(&self) -> QuadraticResult {
        let disc = (self.x_coef * self.x_coef) - (4.0 * self.x_squared_coef * self.constant);
        match disc {
            d if d < 0.0 => {
                QuadraticResult::ImaginaryRoots
            },
            d if d == 0.0 => {
                QuadraticResult::OneRealRoot(-self.x_coef / (2.0 * self.x_squared_coef))
            },
            d => {
                QuadraticResult::TwoRealRoots((
                    (-self.x_coef + d.sqrt()) / (2.0 * self.x_squared_coef),
                    (-self.x_coef - d.sqrt()) / (2.0 * self.x_squared_coef)
                ))
            }

        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn two_real_roots() {
        assert_eq!(Quadratic::new(1.0, 10.0, 24.0).calculate_roots(), QuadraticResult::TwoRealRoots((-4.0, -6.0)));
    }

    #[test]
    fn one_real_root() {
        assert_eq!(Quadratic::new(1.0, 10.0, 25.0).calculate_roots(), QuadraticResult::OneRealRoot(-5.0));
    }

    #[test]
    fn imaginary_roots() {
        assert_eq!(Quadratic::new(1.0, 1.0, 25.0).calculate_roots(), QuadraticResult::ImaginaryRoots);
    }
}

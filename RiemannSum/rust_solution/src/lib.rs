/// This module contains a structure for calculating left, right, and midpoint riemann sums.

// "struct" is short for structure. A struct in rust is basically
// just a big glob information. If you're coming from another programming
// this is often called an "object" or a "record"
//
// For our riemann sum, the information
// we need to solve the problem is:
//
// A function to approximate: func
// a starting point: x0
// an ending point: xf
// the number of rectangles to use: n
//
// We also want to know which kind of riemann sum we are doing (left, right, midpoint)
// For that we use our own list of possibilities called method
//
// The 'pub' keyword just means public. It's there so other people can use our struct
pub struct RiemannSum {
    func: fn(f64) -> f64,
    x0: f64,
    xf: f64,
    n: u64,
    method: Method,
}

// "enum" is short for enumeration. Enums in programming are often a
// convenient ways to define a set number of options. In rust, enums
// are much more powerful than this, but all we need is a simple one
pub enum Method {
    Right,
    Left,
    Midpoint,
}

// "impl" is short for Implementation. This is where we can write behavior associated with our big
// globs of information. You can also write behavior that is associated with more than one struct
// using "traits", but we don't need that for our calculator.
impl RiemannSum {
    // This function is what's often called a 'constructor'. It takes in all the info we need
    // and returns 'Self" which is just an easy way of writing 'RiemannSum'. If you replace the word Self
    // with RiemannSum, this function will still work. This one just passes along the info,
    // but constructors can be used to do calculations for the user as well.
    // You don't have to have a constructor in Rust, instead you can type the name of the struct and
    // the data all in squiggly braces (which is exactly what our constructor does), but they are
    // often convenient.
    pub fn new(func: fn(f64) -> f64, x0: f64, xf: f64, n: u64, method: Method) -> Self {
        RiemannSum {
            func,
            x0,
            xf,
            n,
            method,
        }
    }

    // This is where the magic happens. This function uses all the information in our struct to calculate a riemann sum!
    // The 'self' keyword is our way to access the information stored in our struct
    pub fn calculate(&self) -> f64 {
        // The width of the rectangles = the width of the graph divided by the number of rectangles
        // i.e. If we squeeze 7 rectangles between x=0 and x=21, each rectangle has a width of 3
        let interval = (self.xf - self.x0) / (self.n as f64);

        // We are going to add the area of each rectangle to this variable, and return it at the end
        let mut total_area = 0.0;

        // For a left riemann sum, we start at the very left of the first rectangle
        // For a middle riemann sum, we start in the middle of the first rectangle
        // For a right riemann sum, we start to the right of the first rectangle
        let mut eval_at = match self.method {
            Method::Left => self.x0,
            Method::Midpoint => self.x0 + (interval / 2.0),
            Method::Right => self.x0 + interval,
        };

        // We need to add up n rectangles
        for _ in 0..self.n {
            // The area of a rectangle is its height (func(eval_at)) * its width (interval)
            total_area += (self.func)(eval_at) * interval;

            // After calculating the area of our rectangle, we increase x by the width of the ractangle to take us to the next one
            eval_at += interval;
        }

        total_area
    }
}

#[cfg(test)]
mod tests {
    use super::{Method, RiemannSum};

    #[test]
    fn left_riemman_sum() {
        // Calculate a left riemann sum for y=x^2 from 0 to 3 with n=3 rectangles
        // the "|x| x*x" bit is called a closure. It's a bit like a function, and this one
        // takes in some x and returns x*x which is x^2. Taking advantage of closures makes this
        // calculator very easy to use.
        let sum = RiemannSum::new(|x| x * x, 0.0, 3.0, 3, Method::Left).calculate();

        // Sum should equal (0.0)^2 + (1.0)^2 + (2.0)^2 = 5.0
        assert_eq!(sum, 5.0);
    }

    #[test]
    fn right_riemann_sum() {
        let sum = RiemannSum::new(|x| x * x, 0.0, 3.0, 3, Method::Right).calculate();

        // Sum should equal (1.0)^2 + (2.0)^2 + (3.0)^2 = 14.0
        assert_eq!(sum, 14.0);
    }

    #[test]
    fn midpoint_riemann_sum() {
        let sum = RiemannSum::new(|x| x * x, 0.0, 3.0, 3, Method::Midpoint).calculate();

        // Sum should equal (0.5)^2 + (1.5)^2 + (2.5)^2 = 8.75
        assert_eq!(sum, 8.75);
    }
}

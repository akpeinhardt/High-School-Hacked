#include <stdio.h>

// 'enum' is short for Enumeration. Enums in c are a handy way
// to create a finite list of options. One of the most common ways
// enums are used in C is to create error types.
enum Method {
    Left,
    Right,
    Midpoint
};

// This is the function we plan to perform our riemann sum on.
// In this case, f(x) = x^2
double f(double x) {
    return x * x;
}

// This function actually calculates the riemann sum, and returns a floating point answer
double calculate(double x0, double x1, int n, enum Method method) {
    
    // The width of the rectangle = the width of the graph / the number of rectangles
    // if we do a riemann sum with 7 rectangles from x = 0 to x = 21,
    // then each rectangle is 3 wide
    double interval = (x1 - x0) / (double) n;
    
    
    double evalAt;
    if (method == Left) {
        // for a left riemann sum, we start at the beginning
        evalAt = x0;
    } else if (method == Midpoint) {
        // for a midpoint sum, we move halfway into the rectangle
        evalAt = x0 + (interval / 2.0);
    } else if (method == Right) {
        // for a right sum, we move to the right side of the rectangle
        evalAt = x0 + interval;
    }

    // We are going to use this variable to add up the areas of all the rectangles
    double sum = 0;

    // we add up n rectangles so we iterate n times
    for (int i = 0; i < n; i++) {

        // Evaluate the function at the point and add the result to our total
        sum += f(evalAt) * interval;
        // Go to the nect rectangle
        evalAt += interval;
    }

    return sum;
}

int main() {

    // Most introductory C courses do not cover testing, and C does not have
    // a defacto winner for testing frameworks. So in the spirit of reliving
    // college, we will use a bunch of print statements instead.

    // Evaluate x^2 from 0..3 with n = 3 rectangles as a Left Riemann sum
    // should print 5
    printf("From the left: %lf\n", calculate(0, 3, 3, Left));

    // Evaluate x^2 from 0..3 with n = 3 rectangles as a Right Riemann sum
    // should print 14
    printf("From the right: %lf\n", calculate(0, 3, 3, Right));

    // Evaluate x^2 from 0..3 with n = 3 rectangles as a Midpoint Riemann sum
    // should print 8.75
    printf("From the midpoint: %lf\n", calculate(0, 3, 3, Midpoint));

    return 0;
}
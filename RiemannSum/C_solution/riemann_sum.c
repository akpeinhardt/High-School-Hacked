#include <stdio.h>

enum Method {
    Left,
    Right,
    Midpoint
};

double f(double x) {
    return x * x;
}

double calculate(double x0, double x1, int n, enum Method method) {
    
    double interval = (x1 - x0) / (double) n;
    
    double evalAt;
    if (method == Left) {
        evalAt = x0;
    } else if (method == Midpoint) {
        evalAt = x0 + (interval / 2.0);
    } else if (method == Right) {
        evalAt = x0 + interval;
    }

    double sum = 0;
    for (int i = 0; i < n; i++) {
        sum += f(evalAt);
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
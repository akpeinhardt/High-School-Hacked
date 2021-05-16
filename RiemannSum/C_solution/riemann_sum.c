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
        sum += f(evalAt) * interval;
        evalAt += interval;
    }

    return sum;
}

int main() {

    printf("From the left: %lf\n", calculate(0, 3, 3, Left));
    printf("From the right: %lf\n", calculate(0, 3, 3, Right));
    printf("From the midpoint: %lf\n", calculate(0, 3, 3, Midpoint));

    return 0;
}
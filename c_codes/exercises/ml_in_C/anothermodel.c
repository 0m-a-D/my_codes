#include <stdio.h>

#define LEN sizeof(data) / sizeof(data[0])

struct Tuple {
    int first;
    int second;
};

struct Tuple data[] = {
    {1, 3},
    {2, 5},
    {3, 7},
    {4, 9}
};

float bias = 0.0, opt_weight = 0.0; // Global variables

void training() {
    float sum_x = 0.0, sum_y = 0.0;
    
    for (int i = 0; i < LEN; ++i) {
        sum_x += data[i].first;
        sum_y += data[i].second;
    }
    
    float x_mean = sum_x / LEN;
    float y_mean = sum_y / LEN;
    
    float num = 0.0, den = 0.0;

    for (int i = 0; i < LEN; ++i) {
        num += (data[i].first - x_mean) * (data[i].second - y_mean);
        den += (data[i].first - x_mean) * (data[i].first - x_mean);
    }

    opt_weight = num / den;
    bias = y_mean - opt_weight * x_mean;
}

int optimization_func(int input) {
    int result = opt_weight * input + bias;
    return result;
}

int main() {
    training();
    
    int input;
    printf("Enter a number for prediction: ");
    scanf("%d", &input);
    
    int prediction = optimization_func(input);
    printf("The predicted output is %d\n", prediction);
    
    return 0;
}

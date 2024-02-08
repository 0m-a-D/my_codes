// no learning rate. linear regression model.
#include <stdio.h>
#define LEN sizeof(data) / sizeof(data[0])
struct Tuple
{
    int first;
    int second;
};
struct Tuple data[] = {
    {1, 3},
    {2, 5},
    {3, 7},
    {4, 9}};
// the above data should generate the function **y = 2x + 1**
float bias = 0.00, opt_weight = 0.00; // global variables

void training()
{
    // finding weight and bias --> reinforcement learning
    float weight[LEN];

}

int optimization_func(int input)
{
    /*
    i have optimized function parameters, i.e, weight and bias from training()function.
    use this function to estimate output from test inputs.
    */
    int result = opt_weight * input + bias;
    return result;
}

int main()
{
    // execution
    int num;
    training();
    scanf("%d",&num);
    printf("the predicted output for the given input is %d",optimization_func(num));
    return 0;
}
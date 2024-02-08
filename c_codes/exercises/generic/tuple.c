#include <stdio.h>

struct Tuple {
    int first, second, third, fourth;
};

int main() {
    struct Tuple data[] = {
        {1, 2},
        {2, 4},
        {3, 6},
        {4, 8},
        {2, 3},
        {1,2,69,400}
    };

    // Accessing tuple elements
    printf("First element of the first tuple: %d\n", data[0].first);
    printf("Second element of the second tuple: %d\n", data[1].second);
    printf("fourth element of the sixth tuple: %d\n", data[5].fourth);
    printf("third element of the sixth tuple: %d\n", data[5].third);
    // Modifying tuple elements
    data[2].first = 5;
    data[3].second = 10;
    return 0;
}
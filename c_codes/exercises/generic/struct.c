#include <stdio.h>
#define PI 3.14
struct shape
{
    int side;
    float area;
    float perimeter;
};
float area(int side)
{
    return PI * side * side;
}
int main()
{
    struct shape s;
    printf("enter the value of side: ");
    scanf("%d", &s.side);
    printf("area of the circle is: %f\n", area(s.side));

    // address of these structs
    printf("address of s.side %p\n", &s.side);
    printf("address of s.area %p\n", &s.area);
    printf("address of s.perimeter %p\n", &s.perimeter);
    printf("address of type circle is %p\n", &s);
    int a;
    printf("a address is %p\n",&a);
    return 0;
}
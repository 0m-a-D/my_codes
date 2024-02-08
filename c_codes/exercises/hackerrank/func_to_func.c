#include <stdio.h>

int square(int a)
{
    return a * a;
}

void func(int (*func)(int))
{
    printf("%d\n", func(5));
}

int main()
{
    func(square);
    return 0;
}
#include <stdio.h>
int main()
{
    int num;
    scanf("%d", &num);
    int sum = 0;
    for (int i = 0; i < 5; i++)
    {
        int rem = num % 10;
        int quot = num / 10;
        num = quot;
        sum += rem;
    }
    printf("%d\n", sum);
    return 0;
}
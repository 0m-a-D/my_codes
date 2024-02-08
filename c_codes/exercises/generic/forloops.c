#include <stdio.h>
int main()
{
    const int lines = 10;
    for (int i = 0; i < lines; i++)
    {
        for (int j = 0; j <= i; j++)
        {
            printf("*");
        }
        printf("\n");
    }
    printf("\n");
    for (int i = 10; i > 0; i--)
    {
        for (int column = 0; column < i; column++)
        {
            printf("*");
        }
        printf("\n");
    }
    return 0;
}
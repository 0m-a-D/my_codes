#include <stdio.h>
int main()
{
    int rows = 10;
    while (rows > 0)
    {
        int column = 0;
        while (column < rows)
        {
            printf("*");
            ++column;
        }
        --rows;
        printf("\n");
    }
    printf("\n");

    rows = 10;
    int i = 0;
    while (i < rows)
    {
        int column = 0;
        while (column <= i)
        {
            printf("*");
            ++column;
        }
        ++i;
        printf("\n");
    }

    return 0;
}
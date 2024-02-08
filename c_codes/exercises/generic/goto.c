#include <stdio.h>
int main()
{
    int c = 0;
    printf("using goto\n");
LOOP:
    c += 1;
    if (c < 10)
    {
        printf("%d\n", c);
        goto LOOP;
    }
    else
    {
        printf("current value of c is -> %d\n", c);
        goto HERE;
    }
HERE:
    printf("------------------------------\n");
    printf("using for loop\n");
    for (int i = 1; i < 10; i++)
    {
        printf("%d\n", i);
    }
    printf("current value of c -> %d\n", c);
    return 0;
}
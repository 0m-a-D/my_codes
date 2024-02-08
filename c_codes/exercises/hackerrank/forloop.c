#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

int main()
{
    int a, b;
    scanf("%d\n%d", &a, &b);
    // Complete the code.
    for (int i = a; i <= b; i++)
    {
        if (i > 9 && i % 2 == 0)
        {
            printf("even\n");
        }
        else if (i > 9 && i % 2 != 0)
        {
            printf("odd\n");
        }
        else if (i <= 9)
        {
            char *words[10] = {"zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"};
            printf("%s\n", words[i]);
        }
    }
    return 0;
}
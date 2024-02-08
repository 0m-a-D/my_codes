#include <stdio.h>
#include <stdlib.h>
int main()
{
    int *a = malloc(1 * sizeof(int));
    *a = 10;
    printf("value of a is %d\n", *a);
    printf("address of a is %p\n", &a);
    printf("address where 10 is stored is %p\n", a);
    int **ptr;
    ptr = &a;
    free(a);
    printf("value of a is %d\n", **ptr);
    // ptr is now a dangling pointer!!
    return 0;
}
#include <stdlib.h>
#include<stdio.h>

int main() {
    int* ptr;
    ptr = (int*) malloc(sizeof(int));
// Check if memory allocation was successful
    if (ptr == NULL) {
        printf("Memory allocation failed\n");
        return 1;
    }
    *ptr = 42;
    printf("Value of variable: %d\n", *ptr);
    printf("address of this pointer is: %p\n",ptr);
    free(ptr);
    return 0;
}
/*
 *
 *RAM ARCHITECTURE

-----------------------------STACK
-----------------------------HEAP
-----------------------------GLOBAL/STATIC VARIABLES
-----------------------------MACHINE CODE (.TEXT)
*/
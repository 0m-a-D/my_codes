#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int main(){
    int *i; int a;
    void *v;
    printf("sizeof(int*): %zu\n",sizeof(i));
    printf("sizeof(void*): %zu\n",sizeof(v));
    printf("sizeof(a): %lu\n",sizeof(a));
    return 0;
}

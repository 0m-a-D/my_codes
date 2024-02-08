#include<stdio.h>
#include<stdint.h>
int main(){
    printf("%ld\n",sizeof(size_t));
    printf("%ld\n",sizeof(int));
    printf("%ld\n",sizeof(float));
    printf("%ld\n",sizeof(char));
    printf("%ld\n",sizeof(int*));
    printf("%ld\n",sizeof(float*));
    printf("%ld\n",sizeof(char*));
    size_t a = -9;
    printf("%zu\n",a);

    uint64_t b = -9;
    printf("%zu\n",b);
    return 0;
}
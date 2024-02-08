#include<stdio.h>
int main(){
    int a, *aptr,**ptrptr;
    aptr = &a;
    ptrptr = &aptr;
    printf("address of a -> %p\n",&a);
    printf("address of a -> %p\n",aptr);
    printf("address of aptr -> %p\n",&aptr);
    printf("address of aptr -> %p\n",ptrptr);
    printf("address of ptrptr -> %p\n",&ptrptr);
    return 0;
}
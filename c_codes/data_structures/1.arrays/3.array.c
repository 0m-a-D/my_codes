#include<stdio.h>
#include<stdlib.h>
int main(){
    //another cool way to create an array
    printf("---------ARRAY CREATION IN HEAP---------\n");
    int *arr; int num=3;
    arr = (int*) malloc(num*sizeof(int));
    printf("address of this array is %p\n",arr);

    printf("---------ARRAY CREATION IN STACK--------\n");
    int stack_Arr[4];
    printf("address of this array is %p\n",stack_Arr);
    free(arr);


    int a[3]={1,2,3};
    int b = a;
    return 0;
}
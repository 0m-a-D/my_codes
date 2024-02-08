#include<stdio.h>

int sum(int a,int b){
    int result = a+b;
    return result;
}

int main(){
    int a = 5,b=10;
    int result = sum(a,b);
    printf("sum is %d\n",result);
    printf("another result is %d\n",sum(result,5));
    return 0;
}

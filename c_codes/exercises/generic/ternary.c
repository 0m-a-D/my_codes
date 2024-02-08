#include<stdio.h>
#include<stdlib.h>
#include<string.h>
int main(){
    int a;
    printf("enter your age: ");
    scanf("%d",&a);
    (a>18) ? printf("eligible for voting") : printf("not eligible for voting");
    return 0;
}
